#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>

#include <arpa/inet.h>
#include <netinet/in.h>

#include <getopt.h>

#include <openssl/evp.h>
#include <openssl/hmac.h>

#include "app_error.h"
#include "cmd.h"
#include "robot.h"

#define PORT 21374

///////////////////////////////////////////////////////////////////
// Forward declarations
///////////////////////////////////////////////////////////////////

uint32_t getRobotHandle(char robotID[ROBOT_ID_LEN]);

///////////////////////////////////////////////////////////////////
// Dynamic binary buffer (vector-like) structure
///////////////////////////////////////////////////////////////////

typedef struct {
    uint8_t *data;
    size_t size;
    size_t capacity;
} buffer_t;

void bufferInit(buffer_t *buf) {
    buf->capacity = 128;
    buf->size = 0;
    buf->data = malloc(buf->capacity);
}

void bufferFree(buffer_t *buf) {
    if (buf->data)
        free(buf->data);
    buf->data = NULL;
    buf->size = 0;
    buf->capacity = 0;
}

void bufferAppend(buffer_t *buf, const void *src, size_t len) {
    while (buf->size + len > buf->capacity) {
        buf->capacity *= 2;
        buf->data = realloc(buf->data, buf->capacity);
    }

    memcpy(buf->data + buf->size, src, len);
    buf->size += len;
}

void bufferAppendU32(buffer_t *buf, uint32_t val) {
    bufferAppend(buf, &val, sizeof(val));
}

///////////////////////////////////////////////////////////////////
// Command definition, argument encoder and decoder
///////////////////////////////////////////////////////////////////

typedef struct {
    char *username;
    char *password;
    int32_t robotHandle;
} session_t;

session_t gSession = {0};

typedef struct {
    const char *name;
    const char *args;
    const char *shortDesc;
    const char *longDesc;
    cmd_op_t    op;
    bool        requiresHandle;

    bool (*encode)(buffer_t *buf, int argc, char **argv);
    void (*decode)(const response_t *resp);
} cmd_info_t;

bool encodeRobotHandle(buffer_t *buf, int argc, char **argv) {
    if (argc != 1) {
        fprintf(stderr, "Wrong number of arguments\n");
        return false;
    }

    gSession.robotHandle = getRobotHandle(argv[0]);
    bufferAppendU32(buf, gSession.robotHandle);
    return true;
}

bool encodeMove(buffer_t *buf, int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "Wrong number of arguments\n");
        return false;
    }

    // Convert room name to lowercase
    char *roomName = malloc(strlen(argv[1]) + 1);
    memset(roomName, 0, strlen(argv[1]) + 1);
    strcpy(roomName, argv[1]);
    for (size_t i = 0; i < strlen(roomName); ++i) {
        roomName[i] = tolower(roomName[i]);
    }

    // Convert room name to enum value
    room_id_t room;
    if (strcmp(roomName, "storage") == 0) {
        room = ROOM_STORAGE;
    } else if (strcmp(roomName, "greenhouse") == 0) {
        room = ROOM_GREENHOUSE;
    } else if (strcmp(roomName, "maintenance") == 0) {
        room = ROOM_MAINTENANCE;
    } else if (strcmp(roomName, "office") == 0) {
        room = ROOM_OFFICE;
    } else {
        fprintf(stderr, "Unknown room %s\n", roomName);
        return false;
    }

    // Format payload
    encodeRobotHandle(buf, 1, &argv[0]);
    position_t move = { .roomId = room, .x = atoi(argv[2]), .y = atoi(argv[3]) };
    bufferAppend(buf, &move, sizeof(move));
    return true;
}

void decodeError(const response_t *resp) {
    switch (resp->header.status) {
        case ERR_SUCCESS:
            printf("Success\n");
            break;
        case ERR_MSG_INVALID_HEADER:
            fprintf(stderr, "ERROR: Invalid header\n");
            break;
        case ERR_MSG_INVALID_PROTO:
            fprintf(stderr, "ERROR: Invalid protocol magic\n");
            break;
        case ERR_MSG_AUTH_FAILURE:
            fprintf(stderr, "ERROR: Failed to authenticate\n");
            break;
        case ERR_MSG_ALLOC_FAILURE:
            fprintf(stderr, "ERROR: Server failed to allocate enough memory to handle message\n");
            break;
        case ERR_CMD_INVALID_ARG:
            fprintf(stderr, "ERROR: Invalid command arguments\n");
            break;
        case ERR_CMD_UNKNOWN:
            fprintf(stderr, "ERROR: Unknown command\n");
            break;
        case ERR_ROBOT_NOT_FOUND:
            fprintf(stderr, "ERROR: Robot with ID not found\n");
            break;
        case ERR_ROBOT_UNREACHABLE:
            fprintf(stderr, "ERROR: Robot does not accept commands currently\n");
        case ERR_ROBOT_HANDLE_IN_USE:
            fprintf(stderr, "ERROR: Robot handle currently in use, try again later\n");
            break;
        case ERR_ROBOT_HANDLE_INVALID:
            fprintf(stderr, "ERROR: Tried to use invalid robot handle\n");
            break;
        default:
            fprintf(stderr, "ERROR: Unknown error\n");
    }

    if (resp->header.resultLen > 0) {
        fprintf(stderr, "Reason: %s\n", resp->result);
    }
}

void decodeString(const response_t *resp) {
    if (resp->header.status != ERR_SUCCESS) {
        decodeError(resp);
        return;
    }

    printf("%.*s\n", resp->header.resultLen, resp->result);
}

void decodePosition(const response_t *resp) {
    if (resp->header.status != ERR_SUCCESS) {
        decodeError(resp);
        return;
    }

    if (resp->header.resultLen != sizeof(position_t)) {
        fprintf(stderr, "ERROR: Malformed response\n");
        return;
    }

    position_t *pos = (position_t*)resp->result;
    switch (pos->roomId) {
        case ROOM_STORAGE:
            printf("Room: Storage\n");
            break;
        case ROOM_GREENHOUSE:
            printf("Room: Greenhouse\n");
            break;
        case ROOM_MAINTENANCE:
            printf("Room: Maintenance\n");
            break;
        case ROOM_OFFICE:
            printf("Room: Office\n");
            break;
        default:
            printf("Room: Unknown\n");
    }
    printf("Coordinate: (%f, %f)\n", pos->x, pos->y);
}

// A static array containing all your supported commands
static const cmd_info_t CMD_TABLE[] = {
    {
        "list_robots", "",
        "Retrieve the list of robots monitored",
        "Details: Retrieves the list of robot IDs, which can be passed as command parameter",
        CMD_LIST_ROBOTS, false, NULL, decodeString
    },
    {
        "list_handles", "",
        "Retrieve the state of the robot handles",
        "Details: Retrieves the state of the robot handles, indicating which ones are currently in use or free",
        CMD_LIST_ROBOT_HANDLE_STATE, false, NULL, decodeString
    },
    {
        "get_pos", "<robotID>",
        "Retrieve the current position",
        "Details: Retrieves the current position of the robot, in the format (RoomID, x, y)",
        CMD_GET_POS, true, encodeRobotHandle, decodePosition
    },
    {
        "move", "<robotID> <roomId> <x> <y>",
        "Move to a specific coordinate",
        "Details: Moves the robot to the room <RoomID> at position (x, y).\n"
        "List of possible rooms:\n"
        "\tstorage\n"
        "\tgreenhouse\n"
        "\tmaintenance\n"
        "\toffice",
        CMD_MOVE, true, encodeMove, decodeError
    },
    {
        "start_sprinkler", "<robotID>",
        "Activate the sprinkler",
        "Details: Instruct the robot to activate the sprinkler immediately",
        CMD_START_SPRINKLER, true, encodeRobotHandle, decodeError
    },
    {
        "stop_sprinkler", "<robotID>",
        "Deactivate the sprinkler",
        "Details: Instruct the robot to deactivate the sprinkler immediately",
        CMD_STOP_SPRINKLER, true, encodeRobotHandle, decodeError
    },
    {
        "status", "<robotID>",
        "Get robot status",
        "Details: Get robot status",
        CMD_STATUS, true, encodeRobotHandle, decodeString
    },
    {NULL, NULL, NULL, NULL, CMD_ROBOT_LAST, false, NULL, NULL}
};

void printHelp(const char *progName, const char *subCmd) {
    // Specific Sub-command Help
    if (subCmd != NULL) {
        for (int i = 0; CMD_TABLE[i].name != NULL; i++) {
            if (strcmp(subCmd, CMD_TABLE[i].name) == 0) {
                printf("\nCommand: %s %s\n", CMD_TABLE[i].name, CMD_TABLE[i].args);
                printf("%s\n\n", CMD_TABLE[i].longDesc);
                return;
            }
        }
        printf("Error: Help topic '%s' not found.\n", subCmd);
    }

    // General Help (Fallback)
    printf("\nUsage: %s [--user <username>] [--pass <password>] <command> [args]\n\n", progName);
    printf("For commands requiring authentication, use the --user and --pass arguments\n");
    printf("or use the SCADA_USERNAME and SCADA_PASSWORD environment variables\n\n");
    printf("%-15s %-15s %s\n", "Command", "Arguments", "Description");
    printf("------------------------------------------------------------\n");
    for (int i = 0; CMD_TABLE[i].name != NULL; i++) {
        printf("%-15s %-15s %s\n",
               CMD_TABLE[i].name,
               CMD_TABLE[i].args,
               CMD_TABLE[i].shortDesc);
    }
    printf("\nUse '%s help <command>' for more details on a specific action.\n\n", progName);
}

///////////////////////////////////////////////////////////////////
// Network functions
///////////////////////////////////////////////////////////////////

bool generateSignature(msg_t *msg) {
    EVP_MAC *mac = NULL;
    EVP_MAC_CTX *ctx = NULL;
    uint8_t calculated_hmac[32];
    size_t hmac_len = 0;
    bool success = false;

    // 1. Fetch the HMAC algorithm and create a context
    mac = EVP_MAC_fetch(NULL, "HMAC", NULL);
    ctx = EVP_MAC_CTX_new(mac);

    // 2. Set the underlying hash (SHA256) and the key (Password)
    OSSL_PARAM params[2];
    params[0] = OSSL_PARAM_construct_utf8_string("digest", "SHA256", 0);
    params[1] = OSSL_PARAM_construct_end();

    if (EVP_MAC_init(ctx, msg->password, strlen(msg->password), params) != 1) {
        goto cleanup;
    }

    // 3. Feed the data (Username + Opcode + Payload)
    EVP_MAC_update(ctx, (uint8_t*)msg->username, msg->header.usernameLen);

    //uint16_t net_opcode = htons(msg->header.opcode);
    EVP_MAC_update(ctx, (uint8_t*)&msg->header.op, sizeof(msg->header.op));

    EVP_MAC_update(ctx, msg->payload, msg->header.payloadLen);

    // 4. Finalize and copy into message
    if (EVP_MAC_final(ctx, calculated_hmac, &hmac_len, sizeof(calculated_hmac)) == 1) {
        memcpy(msg->header.signature, calculated_hmac, sizeof(calculated_hmac));
    }

cleanup:
    EVP_MAC_CTX_free(ctx);
    EVP_MAC_free(mac);
    return success;
}

ssize_t recvAll(int sock, void *buf, size_t n, int flags) {
    ssize_t totalReceived = 0;
    while (totalReceived < n) {
        ssize_t received = recv(sock, buf + totalReceived, n - totalReceived, flags);
        if (received < 0) {
            perror("Failed to receive");
            return received;
        } else if (received == 0) {
            return totalReceived;
        }

        totalReceived += received;
    }

    return totalReceived;
}

ssize_t sendAll(int sock, const void *buf, size_t n, int flags) {
    ssize_t totalSent = 0;
    while (totalSent < n) {
        // Always set MSG_NOSIGNAL to avoid crash on remote connection close
        ssize_t sent = send(sock, buf + totalSent, n - totalSent, flags | MSG_NOSIGNAL);
        if (sent < 0) {
            perror("Failed to send");
            return sent;
        } else if (sent == 0) {
            return totalSent;
        }

        totalSent += sent;
    }

    return totalSent;
}

bool sendMessage(const msg_t *msg, response_t *resp) {
    int sock = socket(AF_INET6, SOCK_STREAM, 0);
    if (sock < 0) {
        perror("Socket creation failed");
        return false;
    }

    struct sockaddr_in6 address;
    memset(&address, 0, sizeof(address));
    address.sin6_family = AF_INET6;
#ifdef USE_LOCALHOST_PORT
    address.sin6_addr = in6addr_loopback;
#else
    inet_pton(AF_INET6, "9000:d37e:c40b:8954:216:3eff:fe22:7a3", &address.sin6_addr);
#endif
    address.sin6_port = htons(PORT);

    if (connect(sock, (struct sockaddr*) &address, sizeof(address)) != 0) {
        perror("Failed to connect");
        return false;
    }

    ssize_t sendSize = sendAll(sock, &msg->header, sizeof(msg->header), 0);
    if (sendSize < 0) {
        return false;
    }

    if (msg->username && msg->header.usernameLen != 0)
        sendAll(sock, msg->username, msg->header.usernameLen, 0);
    if (msg->payload && msg->header.payloadLen != 0)
        sendAll(sock, msg->payload, msg->header.payloadLen, 0);

    ssize_t recvSize = recvAll(sock, resp, sizeof(response_header_t), 0);
    if (resp->header.resultLen > 0) {
        resp->result = malloc(resp->header.resultLen);
        recvAll(sock, resp->result, resp->header.resultLen, 0);
    }

    return true;
}

///////////////////////////////////////////////////////////////////
// Authentication command parsing and robot handle handling
///////////////////////////////////////////////////////////////////

uint32_t getRobotHandle(char robotID[ROBOT_ID_LEN]) {
    msg_t msg = {
        .header = {
            .magic = PROTO_MAGIC,
            .op = CMD_ACQUIRE_ROBOT_HANDLE,
            .usernameLen = 0,
            .payloadLen = ROBOT_ID_LEN
        },
        .username = NULL,
        .password = NULL,
        .payload = robotID
    };

    response_t resp;
    if (!sendMessage(&msg, &resp)) {
        fprintf(stderr, "Failed to send message\n");
        exit(1);
    }

    if (resp.header.status != ERR_SUCCESS) {
        decodeError(&resp);
        exit(1);
    }

    uint32_t index;
    memcpy(&index, resp.result, sizeof(index));

    return index;
}

void releaseRobotHandle(uint32_t index) {
    msg_t msg = {
        .header = {
            .magic = PROTO_MAGIC,
            .op = CMD_RELEASE_ROBOT_HANDLE,
            .usernameLen = 0,
            .payloadLen = sizeof(index)
        },
        .username = NULL,
        .password = NULL,
        .payload = malloc(sizeof(index))
    };

    memcpy(msg.payload, &index, sizeof(index));

    response_t resp;
    if (!sendMessage(&msg, &resp)) {
        fprintf(stderr, "Failed to send message\n");
        exit(1);
    }

    if (resp.header.status != ERR_SUCCESS) {
        decodeError(&resp);
        exit(1);
    }
}

void parseArgs(int argc, char **argv, session_t *session) {
    session->username = getenv("SCADA_USERNAME");
    session->password = getenv("SCADA_PASSWORD");

    static struct option longOptions[] = {
        {"user", required_argument, 0, 'u'},
        {"pass", required_argument, 0, 'p'},
        {0, 0, 0, 0}
    };

    int opt;
    while ((opt = getopt_long(argc, argv, "u:p:", longOptions, NULL)) != -1) {
        switch (opt) {
            case 'u':
                session->username = optarg;
                break;
            case 'p':
                session->password = optarg;
                break;
        }
    }
}

int main(int argc, char **argv) {
    // Parse authentication arguments
    parseArgs(argc, argv, &gSession);

    int cmdArgc = argc - optind;
    char **cmdArgv = &argv[optind];

    if (cmdArgc < 1) {
        // No command passed
        printHelp(argv[0], NULL);
        return 1;
    }

    // Check for help or sub-help
    if (strcmp(cmdArgv[0], "help") == 0 || strcmp(cmdArgv[0], "--help") == 0) {
        char *topic = (cmdArgc > 1) ? cmdArgv[1] : NULL;
        printHelp(argv[0], topic);
        return 0;
    }

    // Parse command
    const cmd_info_t *cmd = NULL;
    for (int i = 0; CMD_TABLE[i].name != NULL; i++) {
        if (strcmp(cmdArgv[0], CMD_TABLE[i].name) == 0) {
            cmd = &CMD_TABLE[i];
            break;
        }
    }

    if (!cmd) {
        fprintf(stderr, "Error: Unknown command '%s'.\n", argv[1]);
        printHelp(argv[0], NULL);
        return 1;
    }

    // Parse command arguments
    buffer_t payload;
    bufferInit(&payload);

    if (cmd->requiresHandle) {
        if (!gSession.username || !gSession.password) {
            fprintf(stderr, "Username and password required from args or environment variables\n");
            printHelp(argv[0], NULL);
            exit(1);
        }
    }

    if (cmd->encode) {
        bool res = cmd->encode(&payload, cmdArgc - 1, &cmdArgv[1]);
        if (!res) {
            printHelp(argv[0], cmdArgv[0]);
            bufferFree(&payload);
            return 1;
        }
    }

    // Generate message
    msg_t msg = {
        .header = {
            .magic = PROTO_MAGIC,
            .op = cmd->op,
            .usernameLen = (gSession.username) ? (strlen(gSession.username) + 1) : 0,
            .payloadLen = payload.size
        },
        .username = gSession.username,
        .password = gSession.password,
        .payload = payload.data
    };

    if (cmd->requiresHandle) {
        generateSignature(&msg);
    }

    response_t resp = {0};
    bool res = sendMessage(&msg, &resp);
    bufferFree(&payload);

    if (cmd->decode) {
        cmd->decode(&resp);
    }

    if (cmd->requiresHandle) {
        releaseRobotHandle(gSession.robotHandle);
    }

    return resp.header.status == ERR_SUCCESS;
}

#include <malloc.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <netinet/in.h>

#include "app_error.h"
#include "auth.h"
#include "cmd.h"
#include "robot.h"
#include "scada.h"
#include "threadpool.h"

#define PORT 21374

#define MAX_USERNAME_LEN 32U
#define MAX_PAYLOAD_LEN 32U
#define PASSWORD_LEN 32U

#define min(a,b) (((a)<(b))?(a):(b))
#define max(a,b) (((a)>(b))?(a):(b))

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

void receiveMessage(int sock) {
    // Read the header first and validate the protocol
    msg_t msg = {0};
    response_t resp = {0};

    ssize_t recvLen = recvAll(sock, &msg.header, sizeof(msg.header), 0);
    if (recvLen != sizeof(msg.header)) {
        if (recvLen < 0)
            perror("Communication error while receiving header");
        else
            fprintf(stderr, "Invalid header size %d\n", recvLen);

        resp.header.status = ERR_MSG_INVALID_HEADER;
        sendAll(sock, &resp, sizeof(response_header_t), 0);
        return;
    }

    if (msg.header.magic != PROTO_MAGIC) {
        fprintf(stderr, "Protocol mismatch\n");
        resp.header.status = ERR_MSG_INVALID_PROTO;
        sendAll(sock, &resp, sizeof(response_header_t), 0);
        return;
    }

    // Allocate password buffer (before the username wink wink)
    msg.password = malloc(PASSWORD_LEN);
    memset(msg.password, 0, PASSWORD_LEN);

    // Receive the dynamic payload and dispatch message command
    const size_t usernameLen = min(msg.header.usernameLen, MAX_USERNAME_LEN);
    const size_t payloadLen = min(msg.header.payloadLen, MAX_PAYLOAD_LEN);
    const size_t recvBufLen = usernameLen + payloadLen;

    uint8_t *recvBuf = malloc(recvBufLen);
    memset(recvBuf, 0, recvBufLen);

    if (recvBuf) {
        recvLen = recvAll(sock, recvBuf, recvBufLen, 0);

        msg.username = recvBuf;
        msg.payload = recvBuf + usernameLen;

        getPassword(msg.username, msg.password);
        dispatchCmd(&msg, &resp);

        if (resp.header.status == ERR_SUCCESS) {
            printf("Command %d executed successfully for user %.*s\n", msg.header.op, usernameLen, msg.username);
        } else {
            fprintf(stderr, "Command %d failed for user %.*s\n", msg.header.op, usernameLen, msg.username);
        }

        ssize_t sendLen = sendAll(sock, &resp, sizeof(response_header_t), 0);
        if (resp.result && sendLen > 0) {
            sendAll(sock, resp.result, resp.header.resultLen, 0);
        }
    } else {
        fprintf(stderr, "Allocation error\n");
        resp.header.status = ERR_MSG_ALLOC_FAILURE;
        sendAll(sock, &resp, sizeof(response_header_t), 0);
    }

    if (recvBuf) {
        free(recvBuf);
        recvBuf = NULL;
    }

    if (msg.password) {
        free(msg.password);
        msg.password = NULL;
    }

    if (resp.result) {
        free(resp.result);
        resp.result = NULL;
    }

    close(sock);
}

int main() {
    // Force all allocations to happen on the main heap, not in thread-local arenas
    mallopt(M_ARENA_MAX, 1);

    scadaInitialize();
    threadpoolCreate(16);

    int serverSock = socket(AF_INET6, SOCK_STREAM, 0);
    if (serverSock < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    const int reusePort = 1;
    if (setsockopt(serverSock, SOL_SOCKET, SO_REUSEADDR, &reusePort, sizeof(reusePort)) < 0) {
        perror("setsockopt(SO_REUSEADDR) failed");
        exit(EXIT_FAILURE);
    }

#if SO_REUSEPORT
    if (setsockopt(serverSock, SOL_SOCKET, SO_REUSEPORT, &reusePort, sizeof(reusePort)) < 0) {
        perror("setsockopt(SO_REUSEPORT) failed");
        exit(EXIT_FAILURE);
    }
#endif

    struct sockaddr_in6 address;
    socklen_t addrlen = sizeof(address);

    memset(&address, 0, sizeof(address));
    address.sin6_family = AF_INET6;
#ifdef USE_LOCALHOST_PORT
    address.sin6_addr = in6addr_loopback;
#else
    address.sin6_addr = in6addr_any;
#endif
    address.sin6_port = htons(PORT);

    if (bind(serverSock, (struct sockaddr*) &address, sizeof(address)) < 0) {
        perror("Bind failed");
        exit(EXIT_FAILURE);
    }

    if (listen(serverSock, 5) < 0) {
        perror("Listen failed");
        exit(EXIT_FAILURE);
    }

    printf("Listening on port %d...\n", PORT);
    while (true) {
        int clientSock = accept(serverSock, (struct sockaddr*) &address, &addrlen);
        if (clientSock < 0) {
            perror("Accept failed");
            continue;
        }

        char clientIp[INET6_ADDRSTRLEN];
        inet_ntop(AF_INET6, &address.sin6_addr, clientIp, sizeof(clientIp));
        printf("Connection from: %s\n", clientIp);

        scadaGarbageCollection();
        if (!threadpoolEnqueue((threadpool_task_t){.function = receiveMessage, .sock = clientSock})) {
            perror("Failed to enqueue socket");
            close(clientSock);
        }
    }

    close(serverSock);
    threadpoolDestroy();

    return 0;
}

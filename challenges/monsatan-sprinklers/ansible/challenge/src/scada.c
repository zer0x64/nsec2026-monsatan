#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "cmd.h"
#include "scada.h"

#define EXPIRY 120

// Right now, the logic is one handle per robot ID. This could be changed if needed.
// There are as many handles as there are robots
static robot_handle_t gRobotHandles[NUM_ROBOTS] = {0};

void scadaInitialize() {
    for (size_t i = 0; i < NUM_ROBOTS; ++i) {
        gRobotHandles[i].robot = robotGet(i);
        gRobotHandles[i].expiration = -1;
        pthread_mutex_init(&gRobotHandles[i].lock, NULL);
        gRobotHandles[i].refCount = 0;
    }
}

void scadaGarbageCollection() {
    for (size_t i = 0; i < NUM_ROBOTS; ++i) {
        if (pthread_mutex_trylock(&gRobotHandles[i].lock) == 0) {
            if(robotHandleIsExpired(&gRobotHandles[i]) && gRobotHandles[i].refCount <= 1) {
                robotHandleDestroy(&gRobotHandles[i]);
            }
            pthread_mutex_unlock(&gRobotHandles[i].lock);
        }
    }
}

void robotHandleCreate(robot_handle_t *handle) {
    if (handle) {
        handle->expiration = time(NULL) + EXPIRY;
        handle->refCount = 1;
    }
}

void robotHandleDestroy(robot_handle_t *handle) {
    if (handle) {
        handle->expiration = -1;
        handle->refCount = 0;
    }
}

robot_t* robotHandleBorrow(uint32_t idx) {
    if (idx > NUM_ROBOTS) {
        return NULL;
    }

    pthread_mutex_lock(&gRobotHandles[idx].lock);
    if (gRobotHandles[idx].refCount == 0 || robotHandleIsExpired(&gRobotHandles[idx])) {
        pthread_mutex_unlock(&gRobotHandles[idx].lock);
        return NULL; // Pointer is expired or invalid
    }

    gRobotHandles[idx].refCount++;
    robotHandleRefreshExpiry(&gRobotHandles[idx]);
    pthread_mutex_unlock(&gRobotHandles[idx].lock);

    return gRobotHandles[idx].robot;
}

void robotHandleReturn(uint32_t idx) {
    pthread_mutex_lock(&gRobotHandles[idx].lock);
    if (gRobotHandles[idx].refCount > 1)
        gRobotHandles[idx].refCount--;
    pthread_mutex_unlock(&gRobotHandles[idx].lock);
}

void robotHandleRefreshExpiry(robot_handle_t *handle) {
    if (handle) {
        handle->expiration = (time(NULL) + EXPIRY);
    }
}

bool robotHandleIsExpired(robot_handle_t *handle) {
    if (!handle) {
        return false;
    }

    if (handle->expiration == -1) {
        return false;
    }

    time_t current = time(NULL);
    return current > handle->expiration;
}

static void cmdListRobotIDs(uint8_t* payload, uint32_t len, response_t *resp) {
    const size_t totalLength = NUM_ROBOTS * ROBOT_ID_LEN;

    resp->header.status = ERR_SUCCESS;
    resp->header.resultLen = totalLength;
    resp->result = malloc(resp->header.resultLen);

    // Join robot IDs with newline
    char *ptr = resp->result;
    for(size_t i = 0; i < NUM_ROBOTS; ++i) {
        memcpy(ptr, robotGet(i)->id, ROBOT_ID_LEN);
        ptr[ROBOT_ID_LEN - 1] = '\n'; // Replace null byte by newline
        ptr += ROBOT_ID_LEN;
    }
    resp->result[totalLength - 1] = '\0'; // Except for the last one
}

static void cmdHiddenFlag(uint8_t* payload, uint32_t len, response_t *resp) {
    resp->header.status = ERR_SUCCESS;
    char *flag = getenv("HIDDEN_FLAG");
    if (flag) {
        resp->header.resultLen = strlen(flag) + 2;
        resp->result = malloc(resp->header.resultLen);
        strncpy(resp->result, flag, resp->header.resultLen - 2);
        resp->result[resp->header.resultLen - 2] = '\n';
        resp->result[resp->header.resultLen - 1] = '\0';
    } else {
        char msg[] = "Failed to read hidden flag, contact challenge designer\n";
        resp->header.resultLen = sizeof(msg);
        memcpy(resp->result, flag, resp->header.resultLen);
    }
}

static void cmdListRobotHandlesState(uint8_t* payload, uint32_t len, response_t *resp) {
    char buffer[256] = {0};
    char *ptr = buffer;
    size_t remaining = sizeof(buffer);
    for(size_t i = 0; i < NUM_ROBOTS; ++i) {
        const char *status = (gRobotHandles[i].refCount == 0) ? "Free\n" : "Busy\n";
        int written = snprintf(ptr, remaining, status);
        if (written > 0 && written < remaining) {
            ptr += written;
            remaining -= written;
        }
    }

    resp->header.status = ERR_SUCCESS;
    resp->header.resultLen = strlen(buffer);
    resp->result = malloc(resp->header.resultLen);
    memcpy(resp->result, buffer, resp->header.resultLen);
    resp->result[resp->header.resultLen - 1] = '\0';
}

static void cmdAcquireRobotHandle(uint8_t* payload, uint32_t len, response_t *resp) {
    for (uint32_t i = 0; i < NUM_ROBOTS; ++i) {
        pthread_mutex_lock(&gRobotHandles[i].lock);
        if (strncmp(robotGet(i)->id, payload, len) == 0) {
            if (gRobotHandles[i].refCount == 0) {
                robotHandleCreate(&gRobotHandles[i]);

                resp->header.status = ERR_SUCCESS;
                resp->header.resultLen = sizeof(i);
                resp->result = malloc(resp->header.resultLen);
                memcpy(resp->result, &i, sizeof(i));
            } else {
                resp->header.status = ERR_ROBOT_HANDLE_IN_USE;
                resp->header.resultLen = 0;
            }

            pthread_mutex_unlock(&gRobotHandles[i].lock);
            return;
        }
        pthread_mutex_unlock(&gRobotHandles[i].lock);
    }

    resp->header.status = ERR_ROBOT_NOT_FOUND;
    resp->header.resultLen = 0;
}

static void cmdReleaseRobotHandle(uint8_t* payload, uint32_t len, response_t *resp) {
    uint32_t idx;
    memcpy(&idx, payload, sizeof(idx));

    if (idx > NUM_ROBOTS) {
        resp->header.status = ERR_ROBOT_NOT_FOUND;
        resp->header.resultLen = 0;
        return;
    }

    pthread_mutex_lock(&gRobotHandles[idx].lock);
    if (gRobotHandles[idx].refCount <= 1) {
        robotHandleDestroy(&gRobotHandles[idx]);
    }
    pthread_mutex_unlock(&gRobotHandles[idx].lock);

    resp->header.status = ERR_SUCCESS;
    resp->header.resultLen = 0;
}

typedef struct {
    cmd_op_t op;
    void (*handler)(uint8_t* payload, uint32_t len, response_t *resp);
} cmd_t;

static const cmd_t dispatchTable[] = {
    {CMD_LIST_ROBOTS, cmdListRobotIDs},
    {CMD_DEPRECATED, cmdHiddenFlag},
    {CMD_LIST_ROBOT_HANDLE_STATE, cmdListRobotHandlesState},
    {CMD_ACQUIRE_ROBOT_HANDLE, cmdAcquireRobotHandle},
    {CMD_RELEASE_ROBOT_HANDLE, cmdReleaseRobotHandle}
};

void scadaDispatchCmd(const msg_t *msg, response_t *resp) {
    cmd_op_t cmdOp = msg->header.op;
    uint8_t *payload = msg->payload;
    uint32_t len = msg->header.payloadLen;

    resp->header.op = cmdOp;
    if (cmdOp > CMD_SCADA_FIRST && cmdOp < CMD_SCADA_LAST) {
        size_t dispatchIndex = cmdOp - CMD_SCADA_FIRST - 1;
        dispatchTable[dispatchIndex].handler(payload, len, resp);
    } else {
        fprintf(stderr, "Unknown scada command: %d\n", cmdOp);
        resp->header.status = ERR_CMD_UNKNOWN;
        resp->header.resultLen = 0;
    }
}

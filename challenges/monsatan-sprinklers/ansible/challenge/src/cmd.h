#ifndef CMD_H
#define CMD_H

#include <stdlib.h>
#include <stdint.h>
#include <unistd.h>

#include "app_error.h"

// PEST
#define PROTO_MAGIC 0x50455354

typedef enum {
    // SCADA commands (non-authenticated)
    CMD_SCADA_FIRST = 1000,
    CMD_LIST_ROBOTS,
    CMD_DEPRECATED,
    CMD_LIST_ROBOT_HANDLE_STATE,
    CMD_ACQUIRE_ROBOT_HANDLE,
    CMD_RELEASE_ROBOT_HANDLE,
    CMD_SCADA_LAST,

    // Robot commands (authenticated)
    CMD_ROBOT_FIRST = 2000,
    CMD_GET_POS,
    CMD_MOVE,
    CMD_START_SPRINKLER,
    CMD_STOP_SPRINKLER,
    CMD_STATUS,
    CMD_ROBOT_LAST
} cmd_op_t;

#pragma pack(push, 1)
typedef struct {
    uint32_t magic;
    uint8_t signature[32];
    cmd_op_t op;
    uint32_t usernameLen;
    uint32_t payloadLen;
} msg_header_t;
#pragma pack(pop)

typedef struct {
    msg_header_t header;
    char *username;
    char *password;
    uint8_t *payload;
} msg_t;

#pragma pack(push, 1)
typedef struct {
    app_error_t status;
    cmd_op_t op;
    uint32_t resultLen;
} response_header_t;
#pragma pack(pop)

typedef struct {
    response_header_t header;
    uint8_t *result;
} response_t;

void dispatchCmd(const msg_t *msg, response_t *resp);

#endif

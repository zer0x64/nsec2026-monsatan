#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "robot.h"
#include "scada.h"

#define MAX_REPORT_LEN 256U
#define ROBOT_VERSION 0x0702A139

#define PLANTER_SIZE 0.8f
#define ROBOT_SIZE 2.0f
#define SAFE_DISTANCE 1.5f
#define WATER_REFILL_POSITION(offset) { ROOM_STORAGE, 22.5f, 1.15f + offset * (ROBOT_SIZE + SAFE_DISTANCE) }
#define PESTICIDE_REFILL_POSITION(offset) { ROOM_STORAGE, 27.5f, 1.15f + offset * (ROBOT_SIZE + SAFE_DISTANCE) }
#define REPAIR_STATION_POSITION { ROOM_MAINTENANCE, 0.0f, 0.0f } // location is offline, other than room
#define RECHARGE_STATION_POSITION(offset) { ROOM_MAINTENANCE, 10.25f + offset * (ROBOT_SIZE + SAFE_DISTANCE), 1.15f }
#define GREENHOUSE_POSITION(lane, offset) { ROOM_GREENHOUSE, 6.0f + offset, 6.0f + 2 * lane * PLANTER_SIZE + lane * ROBOT_SIZE / 2.0f }

static robot_t gRobots[NUM_ROBOTS] = {
    {
        .id = "iuVAGCXzw4f5",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_MAINTENANCE,
        .position = REPAIR_STATION_POSITION,
        .sprinkerActive = false,
        .waterLevel = 0,
        .pesticideLevel = 0
    },
    {
        .id = "Zg67uT3WNZfZ",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_IDLE,
        .position = RECHARGE_STATION_POSITION(0),
        .sprinkerActive = false,
        .waterLevel = 0,
        .pesticideLevel = 0
    },
    {
        .id = "4a6PX5K5Aee5",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_OFFLINE,
        .position = { ROOM_STORAGE, 1.709f, 60.164f },
        .sprinkerActive = false,
        .waterLevel = 10000,
        .pesticideLevel = 5000
    },
    {
        .id = "NWe1P2Nxm4La",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_IDLE,
        .position = RECHARGE_STATION_POSITION(1),
        .sprinkerActive = false,
        .waterLevel = 0,
        .pesticideLevel = 0
    },
    {
        .id = "LQfA8WslFhR0",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_MAINTENANCE,
        .position = REPAIR_STATION_POSITION,
        .sprinkerActive = false,
        .waterLevel = 0,
        .pesticideLevel = 0
    },
    {
        .id = "e045J865CBCW",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = WATER_REFILL_POSITION(0),
        .sprinkerActive = false,
        .waterLevel = 0,
        .pesticideLevel = 3891
    },
    {
        .id = "T801h59jM5Oy",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = PESTICIDE_REFILL_POSITION(0),
        .sprinkerActive = false,
        .waterLevel = 77589,
        .pesticideLevel = 0
    },
    {
        .id = "q1Iv0JL1n2fN",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = GREENHOUSE_POSITION(0, 0.31f),
        .sprinkerActive = true,
        .waterLevel = 64251,
        .pesticideLevel = 18973
    },
    {
        .id = "GGS7Yf93RjGx",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = GREENHOUSE_POSITION(1, 6.874f),
        .sprinkerActive = true,
        .waterLevel = 4102,
        .pesticideLevel = 307,
    },
    {
        .id = "J3yA5McUOO2K",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = PESTICIDE_REFILL_POSITION(1),
        .sprinkerActive = false,
        .waterLevel = 86451,
        .pesticideLevel = 0
    },
    {
        .id = "lVkNsWu0kk2l",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = GREENHOUSE_POSITION(2, 20.086f),
        .sprinkerActive = true,
        .waterLevel = 76041,
        .pesticideLevel = 80057
    },
    {
        .id = "1L5uCs32Js8t",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_IDLE,
        .position = { ROOM_GREENHOUSE, 24.5f, 1.33f },
        .sprinkerActive = false,
        .waterLevel = 104069,
        .pesticideLevel = 30903
    },
    {
        .id = "YuIZbfb6rAae",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = GREENHOUSE_POSITION(3, 3.653f),
        .sprinkerActive = true,
        .waterLevel = 71,
        .pesticideLevel = 897
    },
    {
        .id = "pa6GMLOjc4bM",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = WATER_REFILL_POSITION(1),
        .sprinkerActive = false,
        .waterLevel = 0,
        .pesticideLevel = 9654
    },
    {
        .id = "h9YC3oWKzZ3X",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = WATER_REFILL_POSITION(2),
        .sprinkerActive = false,
        .waterLevel = 0,
        .pesticideLevel = 0
    },
    {
        .id = "HZrBioSHQbmb",
        .version = ROBOT_VERSION,
        .status = ROBOT_STATUS_HEALTHY,
        .position = GREENHOUSE_POSITION(4, 16.743f),
        .sprinkerActive = true,
        .waterLevel = 100240,
        .pesticideLevel = 5046
    },
};

robot_t* robotGet(size_t idx) {
    if (idx > NUM_ROBOTS) {
        return NULL;
    } else {
        return &gRobots[idx];
    }
}

static void cmdGetPos(robot_t *robot, uint8_t *payload, uint32_t len, response_t *resp) {
    if (robot->status == ROBOT_STATUS_OFFLINE || robot->status == ROBOT_STATUS_EMERGENCY_LOCKDOWN) {
        resp->header.status = ERR_ROBOT_UNREACHABLE;
        resp->header.resultLen = 0;
    } else {
        resp->header.status = ERR_SUCCESS;
        resp->header.resultLen = sizeof(robot->position);
        resp->result = malloc(resp->header.resultLen);
        memcpy(resp->result, &robot->position, resp->header.resultLen);
    }
}

static void cmdMove(robot_t *robot, uint8_t *payload, uint32_t len, response_t *resp) {
    if (robot->status == ROBOT_STATUS_OFFLINE || robot->status == ROBOT_STATUS_EMERGENCY_LOCKDOWN) {
        resp->header.status = ERR_ROBOT_UNREACHABLE;
    } else if (len != sizeof(robot->position)) {
        resp->header.status = ERR_CMD_INVALID_ARG;
    } else {
        memcpy(&robot->position, payload, sizeof(robot->position));
        resp->header.status = ERR_SUCCESS;
    }
    resp->header.resultLen = 0;
}

static void cmdStartSprinkler(robot_t *robot, uint8_t *payload, uint32_t len, response_t *resp) {
    if (robot->status == ROBOT_STATUS_OFFLINE || robot->status == ROBOT_STATUS_EMERGENCY_LOCKDOWN) {
        resp->header.status = ERR_ROBOT_UNREACHABLE;
    } else {
        robot->sprinkerActive = true;
        resp->header.status = ERR_SUCCESS;

        // Flag trigger: Activating the sprinklers in the office. For theming, the admins would trigger an
        // emergency lockdown on the robot, which would be noticeable on the following commands.
        if (robot->position.roomId == ROOM_OFFICE) {
            robot->status = ROBOT_STATUS_EMERGENCY_LOCKDOWN;
        }
    }

    resp->header.resultLen = 0;
}

static void cmdStopSprinkler(robot_t *robot, uint8_t *payload, uint32_t len, response_t *resp) {
    if (robot->status == ROBOT_STATUS_OFFLINE || robot->status == ROBOT_STATUS_EMERGENCY_LOCKDOWN) {
        resp->header.status = ERR_ROBOT_UNREACHABLE;
    } else {
        robot->sprinkerActive = false;
        resp->header.status = ERR_SUCCESS;
    }

    resp->header.resultLen = 0;
}

static void cmdStatus(robot_t *robot, uint8_t *payload, uint32_t len, response_t *resp) {
    char *statusString;
    switch (robot->status) {
        case ROBOT_STATUS_OFFLINE:
            statusString = "OFFLINE";
            break;
        case ROBOT_STATUS_IDLE:
            statusString = "IDLE";
            break;
        case ROBOT_STATUS_HEALTHY:
            statusString = "HEALTHY";
            break;
        case ROBOT_STATUS_MAINTENANCE:
            statusString = "MAINTENANCE";
            break;
        case ROBOT_STATUS_EMERGENCY_LOCKDOWN:
            statusString = "EMERGENCY LOCKDOWN";
            break;
    }

    char *reportString = malloc(MAX_REPORT_LEN);
    memset(reportString, 0, MAX_REPORT_LEN);
    snprintf(reportString, MAX_REPORT_LEN, 
        "STATUS           : %s\n"
        "SPRINKLER ACTIVE : %s\n"
        "WATER LEVEL      : %u\n"
        "PESTICIDE LEVEL  : %u\n",
        statusString, (robot->sprinkerActive) ? "YES" : "NO", robot->waterLevel, robot->pesticideLevel
    );

    if (robot->status == ROBOT_STATUS_EMERGENCY_LOCKDOWN) {
        char *flag = getenv("FLAG");
        if (flag) {
            snprintf(
                reportString + strlen(reportString), MAX_REPORT_LEN - strlen(reportString),
                "FATAL ERROR: %s\n", flag
            );
        } else {
            snprintf(
                reportString + strlen(reportString), MAX_REPORT_LEN - strlen(reportString),
                "FATAL ERROR: Unable to read flag, contact challenge designer\n"
            );
        }
    }

    resp->header.status = ERR_SUCCESS;
    resp->header.resultLen = strlen(reportString) + 1;
    resp->result = malloc(resp->header.resultLen);
    memcpy(resp->result, reportString, resp->header.resultLen);

    free(reportString);
}

typedef struct {
    cmd_op_t op;
    void (*handler)(robot_t *robot, uint8_t *payload, uint32_t len, response_t *resp);
} cmd_t;

static const cmd_t dispatchTable[] = {
    {CMD_GET_POS, cmdGetPos},
    {CMD_MOVE, cmdMove},
    {CMD_START_SPRINKLER, cmdStartSprinkler},
    {CMD_STOP_SPRINKLER, cmdStopSprinkler},
    {CMD_STATUS, cmdStatus}
};

void robotDispatchCmd(const msg_t *msg, response_t *resp) {
    cmd_op_t cmdOp = msg->header.op;
    uint8_t *payload = msg->payload;
    uint32_t len = msg->header.payloadLen;

    uint32_t index;
    memcpy(&index, payload, sizeof(index));

    resp->header.op = cmdOp;

    robot_t *robot = robotHandleBorrow(index);
    if (!robot) {
        fprintf(stderr, "Unable to borrow robot handle %d\n", index);
        resp->header.status = ERR_ROBOT_HANDLE_INVALID;
        resp->header.resultLen = 0;
        return;
    }

    if (cmdOp > CMD_ROBOT_FIRST && cmdOp < CMD_ROBOT_LAST) {
        size_t dispatchIndex = cmdOp - CMD_ROBOT_FIRST - 1;
        dispatchTable[dispatchIndex].handler(robot, payload + sizeof(index), len - sizeof(index), resp);
    } else {
        fprintf(stderr, "Unknown robot command: %d\n", cmdOp);
        resp->header.status = ERR_CMD_UNKNOWN;
        resp->header.resultLen = 0;
    }

    robotHandleReturn(index);
}

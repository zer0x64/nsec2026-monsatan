#ifndef ROBOT_H
#define ROBOT_H

#include "cmd.h"

#define ROBOT_ID_LEN 13

typedef enum {
    ROOM_STORAGE,
    ROOM_GREENHOUSE,
    ROOM_MAINTENANCE,
    ROOM_OFFICE,
} room_id_t;

typedef struct {
    room_id_t roomId;
    float x; // m
    float y; // m
} position_t;

typedef enum {
    ROBOT_STATUS_OFFLINE,
    ROBOT_STATUS_IDLE,
    ROBOT_STATUS_HEALTHY,
    ROBOT_STATUS_MAINTENANCE,
    ROBOT_STATUS_EMERGENCY_LOCKDOWN,
} robot_status_t;

typedef struct {
    char id[ROBOT_ID_LEN];
    uint32_t version;

    robot_status_t status;
    position_t position;
    bool sprinkerActive;
    uint32_t waterLevel; // ml
    uint32_t pesticideLevel; // ml
} robot_t;

robot_t* robotGet(size_t idx);
void robotDispatchCmd(const msg_t *msg, response_t *resp);

#endif

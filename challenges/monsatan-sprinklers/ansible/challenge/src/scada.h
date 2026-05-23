#ifndef SCADA_H
#define SCADA_H

#include <stdbool.h>
#include <stdint.h>
#include <pthread.h>
#include <time.h>

#include "cmd.h"
#include "robot.h"

#define NUM_ROBOTS 16

typedef struct {
    robot_t *robot;
    time_t expiration;
    pthread_mutex_t lock;
    uint32_t refCount;
} robot_handle_t;

void robotHandleCreate(robot_handle_t *handle);
void robotHandleDestroy(robot_handle_t *handle);
void robotHandleRefreshExpiry(robot_handle_t *handle);
bool robotHandleIsExpired(robot_handle_t *handle);

robot_t* robotHandleBorrow(uint32_t idx);
void robotHandleReturn(uint32_t idx);

void scadaInitialize();
void scadaGarbageCollection();
void scadaDispatchCmd(const msg_t *msg, response_t *resp);

#endif

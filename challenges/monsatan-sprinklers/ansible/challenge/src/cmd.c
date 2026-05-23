#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "app_error.h"
#include "auth.h"
#include "cmd.h"
#include "robot.h"
#include "scada.h"

void dispatchCmd(const msg_t *msg, response_t *resp) {
    if (msg->header.op > CMD_SCADA_FIRST && msg->header.op < CMD_SCADA_LAST) {
        scadaDispatchCmd(msg, resp);
    } else if (msg->header.op > CMD_ROBOT_FIRST && msg->header.op < CMD_ROBOT_LAST) {
        if (verifyIntegrity(msg)) {
            robotDispatchCmd(msg, resp);
        } else {
            fprintf(stderr, "Message authentication failed\n");

            resp->header.status = ERR_MSG_AUTH_FAILURE;
            resp->header.resultLen = msg->header.usernameLen; // Oops
            resp->result = malloc(resp->header.resultLen);
            memcpy(resp->result, msg->username, msg->header.usernameLen); // Oops
        }
    } else {
        fprintf(stderr, "Unknown command: %d\n", msg->header.op);
        resp->header.status = ERR_CMD_UNKNOWN;
        resp->header.resultLen = 0;
    }
}

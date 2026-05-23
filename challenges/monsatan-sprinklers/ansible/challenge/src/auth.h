#ifndef AUTH_H
#define AUTH_H

#include "cmd.h"

void getPassword(const char *username, char *password);
bool verifyIntegrity(const msg_t *msg);

#endif

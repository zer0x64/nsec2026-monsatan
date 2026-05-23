#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <openssl/evp.h>
#include <openssl/hmac.h>

#include "auth.h"

#define MAX_USERNAME_LEN 32U
#define MAX_PAYLOAD_LEN 32U
#define PASSWORD_LEN 32U

#define min(a,b) (((a)<(b))?(a):(b))

void getPassword(const char *username, char *password) {
#ifdef USE_LOCALHOST_PORT
    // Testing with local demo password
    strcpy(password, "WeLoveBurningTires");
#else
    FILE *fp = fopen("credentials.txt", "r");
    if (!fp) {
        perror("Failed to open credentials file");
        return;
    }

    setvbuf(fp, NULL, _IONBF, 0);

    char line[MAX_USERNAME_LEN + PASSWORD_LEN + 2];
    bool found = false;

    while (fgets(line, sizeof(line), fp)) {
        line[strcspn(line, "\n")] = 0;
        char *user = strtok(line, ":");
        char *pass = strtok(NULL, ":");

        if (user && pass && strcmp(user, username) == 0) {
            strncpy(password, pass, PASSWORD_LEN);
            found = 1;
            break;
        }

        memset(line, 0, sizeof(line));
    }

    fclose(fp);
    fp = NULL;
#endif
}

bool verifyIntegrity(const msg_t *msg) {
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
    EVP_MAC_update(ctx, (uint8_t*)msg->username, min(msg->header.usernameLen, MAX_USERNAME_LEN));
    
    //uint16_t net_opcode = htons(msg->header.opcode);
    EVP_MAC_update(ctx, (uint8_t*)&msg->header.op, sizeof(msg->header.op));
    
    EVP_MAC_update(ctx, msg->payload, min(msg->header.payloadLen, MAX_PAYLOAD_LEN));

    // 4. Finalize and Compare
    if (EVP_MAC_final(ctx, calculated_hmac, &hmac_len, sizeof(calculated_hmac)) == 1) {
        success = (CRYPTO_memcmp(calculated_hmac, msg->header.signature, sizeof(calculated_hmac)) == 0);
    }

cleanup:
    EVP_MAC_CTX_free(ctx);
    EVP_MAC_free(mac);
    return success;
}

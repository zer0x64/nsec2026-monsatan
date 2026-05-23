# monsatan-mailserver

## Description

A mail server with support for sandboxed/signed webassembly/wasi plugins. The participant needs to register their account and forge malicious plugins highjacking signed plugins' permissions by computing a collision on the CRC32 checksum.

## Theme

The finale of the monsatan's track, you need to hack into Johann Elke's inbox(CEO of Monsatan) and doxx him.

## Solutions

Note: To compile my plugins, do
```
cargo build --release --target wasm32-wasip2
```
after installing the `wasm32-wasip2` target.

### Flag 1

See `solve/collision_finder` and `solve/env-plugin`. Basically:
1. Download the approved plugins from the server.
2. Write a malicious plugin that writes all environment variables to the email's body.
3. Compute a collision on the CRC32 checksum so that the malicious plugin's CRC32 checksum matches an approved plugin's CRC32 checksum with env permissions(domain-plugin.tar.gz).
4. Upload the malicious plugin to the server and send to yourself an email with it attached.
5. This leak the database' connection string and the flag is the password to the database.

### Flag 2

See `solve/db-plugin`. For this, you need to write a malicious plugin that dumps the database contents and writes them to the email's body. The difficulty is that there are no mysql driver that just works with wasip2, so you need to write your own. It isn't as bad as it sounds because mysql supports text-based query packets natively, but this is still not a trivial task.
From there, you could leak the emails or better yet, leak the JWT secret and forge a token to gain access to the CEO's inbox.

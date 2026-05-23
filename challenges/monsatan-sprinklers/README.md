# monsatan-sprinklers

## Description

The sprinkler robots in the greenhouses can be controlled and monitored remotely through a custom TCP protocol, and it turns out their firmware is vulnerable to a sort of Heartbleed attack.

## Theme

There are robots everywhere in the greenhouses, vaporising pesticides and water over the crops. Our vigilante group wants to turn these robots around to cause some damage in the offices.

## Solutions

### Flag 1

This is more of a side-quest flag, there is a hidden command which is not used in the CLI. The CLI binary still has debug symbols, so while reversing it, it's possible to see that the command IDs for SCADA (unauthenticated) commands jump one number (skips over 1002). Also, the enum names all appear in the strings, even if that specific one isn't used at all in the code. Search for CMD_DEPRECATED.

Manually calling that command will give the flag. `solve.py` does it if you want a reference.

### Flag 2

See `solve.py`. The CLI binary still has debug symbols and needs to be reversed to find the target URL and message format.
The basic idea is to multithread a bunch of allocation/deallocation on the heap using handles to create chaos in the heap. After that, you can trigger a hearthbleed-like vulnerability by causing a mismatch between the username length field and the actual username length. The server will return an error like "Invalid login for user <username>" which will leak a lot of bytes and hopefully the logins of all users.

# monsatan-orders

## Description

This is an authorization bypass on the internal Monsatan orders portal.

## Theme

The goal here is to pwn the portal and cancel all pending pesticide orders. It is part of the disruption phase of the Monsatan track.

## Solutions

### Flag 1

You can log in using the credentials provided on the discourse post. From there, you can log in as an unprivileged user while looking at the requests.  
The thing to notice is that the JWT returned by the login endpoint does not contains a signature. This is because the JWT is not signed by the server, but rather by the client itself as a result of client/server confusion(the app uses Leptos, a Rust full-stack web framework where client and server share the same codebase).  
From there, there are two ways to forge a valid JWT:  
1. The easy way is to intercept the response from the login endpoint and modify the claims before it reaches the client using caido/burp. After that the client takes care of signing the JWT.
2. The hard way is to reverse engineer the webassembly client code and find the hardcoded signature key used to sign the JWT.  

Once you're able to forge a valid JWT as admin@monsatan.ctf, you can use it to get the flag at /flag.  
Common mistake: When modifying the JWT claims, make sure that you only modify the claims and not header. The format of a JWT is `header.payload.signature`, so if you're using an online JWT editor you may accidentally modify the header(specifically the `alg` field).

#! /usr/bin/env python3

# Standard library import
import os
import socket
from http.server import BaseHTTPRequestHandler, HTTPServer


class RestartHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        os.system("touch /tmp/restart.sig")
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"Restart signal received.")


class HTTPServerV6(HTTPServer):
    address_family = socket.AF_INET6

if __name__ == "__main__":
    server = HTTPServerV6(('::', 8888), RestartHandler)
    server.serve_forever()

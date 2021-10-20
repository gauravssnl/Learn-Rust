#!/usr/bin/env python
import http.server

port = 8000
print("Running on port {}".format(port))

http.server.SimpleHTTPRequestHandler.extensions_map[".wasm"] = "application/wasm"
httpd = http.server.HTTPServer(
    ("localhost", port), http.server.SimpleHTTPRequestHandler
)

print(
    'Mapping ".wasm" to "{}"'.format(
        http.server.SimpleHTTPRequestHandler.extensions_map[".wasm"]
    )
)
httpd.serve_forever()

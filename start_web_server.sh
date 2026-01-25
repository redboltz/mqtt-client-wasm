#!/bin/sh

PORT=${1:-8080}

echo "Starting web server on port $PORT"
echo "Access: http://localhost:$PORT"
echo ""

cd www

python3 -c "
import http.server
import socketserver

PORT = $PORT

class WasmHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for local development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        super().end_headers()

# Add WASM MIME type
WasmHandler.extensions_map['.wasm'] = 'application/wasm'
WasmHandler.extensions_map['.js'] = 'application/javascript'
WasmHandler.extensions_map['.mjs'] = 'application/javascript'

class ReuseAddrTCPServer(socketserver.TCPServer):
    allow_reuse_address = True

with ReuseAddrTCPServer(('', PORT), WasmHandler) as httpd:
    print(f'Serving on port {PORT}')
    httpd.serve_forever()
"

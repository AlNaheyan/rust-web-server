### may 6:
- Start server – Listens for TCP connections on 127.0.0.1:7878.

- Accept request – Reads the first 1024 bytes from the client’s HTTP request.

- Check route – If it's GET /, prepares to serve main.html.

- Fallback – If not GET /, prepares to serve 404.html with a 404 Not Found.

- Send response – Builds and sends an HTTP response with content and proper headers.


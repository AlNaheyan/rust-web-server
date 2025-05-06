use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use rust_server::ThreadPool;

fn main() {
    // Bind to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // request patterns
    let get_root  = b"GET / HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";

    // pick status line + path based on request
    let (status_line, filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK", "pages/main.html")
    } else if buffer.starts_with(get_sleep) {
        // simulate slow work
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "pages/main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };

    // read the file (or show an error page if it fails)
    let content = fs::read_to_string(filename)
        .unwrap_or_else(|_| "<h1>Oops! File not found.</h1>".to_string());

    // build and send the response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

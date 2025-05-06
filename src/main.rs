use std::fs;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::prelude::*;
fn main() {
    // Listenr bound (localhost)
    let listen = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listen.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
// Currenly only checks for GET /
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // Hard Coded 
    let get = b"GET / HTTP/1.1\r\n";
    
    if buffer.starts_with(get) {
        let content = fs::read_to_string("main.html").unwrap();
        let res = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);
    
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("404.html").unwrap();
        let res = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",status_line, content.len(), content
        );

        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}

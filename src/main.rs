use std::net::TcpListener;
fn main() {
    let listen = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listen.incoming() {
        let stream = stream.unwrap();

        println!("connection found");
    }
}

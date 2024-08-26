use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;


fn main() -> std::io::Result<()> {
    const IP: &str = "127.0.0.1";
    const PORT: &str = "7878";
    let listener = TcpListener::bind(format!("{}:{}", IP, PORT)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

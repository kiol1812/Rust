use std::fs;
use std::net::{
    TcpListener,TcpStream
};
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
    } else {
    }
    let (filename, status) = {
        if buffer.starts_with(get) {
            ("index.html", "200 OK")
        } else {
            ("404.html", "404 NOT FOUND")
        }
    };
    let contents  = fs::read_to_string(format!("pages/{}", filename)).unwrap();
    let response = format!(
        "HTTP/1.1 {} \r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
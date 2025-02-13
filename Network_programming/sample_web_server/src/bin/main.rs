use std::net::{
    TcpListener,TcpStream
};
use std::{
    fs, thread
};
use std::time::Duration;
use std::io::prelude::*;

use multi_threaded_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream)
        });
    }
    println!("Shutdown.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get_index = b"GET / HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";
    let (filename, status) = {
        if buffer.starts_with(get_index) {
            ("index.html", "200 OK")
        } else if buffer.starts_with(get_sleep) {
            thread::sleep(Duration::from_secs(5));
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
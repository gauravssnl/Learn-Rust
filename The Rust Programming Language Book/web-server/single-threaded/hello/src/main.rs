use std::fs;
use std::io::prelude::{Read, Write}; // we can use "use std::io::prelude::*" to make it easy
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
        println!("Stream: \n{:?}", stream);
        println!("{}", "-".repeat(80));
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // type [u8]
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
         // Simulating a Slow Request in the Current Server Implementation
         std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    println!("Response to be sent to client :\n{}", response);
    // send the response to client
    stream.write(response.as_bytes()).unwrap();
    // Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
    // flush will wait and prevent the program from continuing until all the bytes are written to the connection;
    // TcpStream contains an internal buffer to minimize calls to the underlying operating system.
    stream.flush().unwrap();
}

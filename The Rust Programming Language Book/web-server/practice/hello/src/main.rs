use std::net::TcpStream;     
use std::io::prelude::*;
fn main() {    
   let mut socket = TcpStream::connect("www.google.com:80").unwrap(); // Google Host    
   let bytes_written = socket.write(b"GET / HTTP/1.0\r\nHost:www.google.com\r\n\r\n").unwrap();                            
   println!("Bytes written: {}", bytes_written);      
   let mut buffer = vec![];                        
   let bytes_read = socket.read_to_end(&mut buffer).unwrap();        
   println!("Bytes read: {}",bytes_read);     
   println!("Response:\n{}", String::from_utf8_lossy(&buffer));                    
}  


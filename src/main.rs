use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer);
    
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server launched to 127.0.0.1:8080");

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
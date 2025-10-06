mod threadpool;
mod router;
mod handlers;
mod http;
mod websockets;
mod middlewares;
mod ratelimit;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::{Arc};

use threadpool::ThreadPool;
use crate::http::HttpRequest;
use router::Router;
use handlers::{hello_handler,home_handler,signup_handler,login_handler};
use websockets::{websocket_handshake,read_frame,send_frame};
use middlewares::{require_auth};
use ratelimit::RateLimiter;


fn parse_request(buffer:&[u8]) -> HttpRequest {
    let request = String::from_utf8_lossy(&buffer);

    let request_line = request.lines().next().unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();

    let mut headers = std::collections::HashMap::new();

    for line in request.lines().skip(1) {
        if line.is_empty() { break; }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    let body = request.split("\r\n\r\n").nth(1).unwrap_or("").trim().to_string();

    HttpRequest { method, path, version, headers, body }
}

fn handle_connection(mut stream:TcpStream,router:&Router){
    let mut buffer = [0;512];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let req = parse_request(&buffer[..bytes_read]);

    let limiter = Arc::new(RateLimiter::new(5,10));


    // If websocket

    if let Some(upgrade) = req.headers.get("Upgrade") {
        if upgrade.to_lowercase() == "websocket" {
            if let Some(key) = req.headers.get("Sec-WebSocket-Key") {
                if req.path != "/" {
                    return;
                }
                let username = match req.get_cookie("user") {
                    Some(u) => u,
                    None => return,
                };
                // Middleware
                if let Err(resp) = require_auth(&req) {
                    stream.write(resp.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    return;
                }
                websockets::websocket_handshake(&mut stream, key);

                loop {
                    if let Some(msg) = websockets::read_frame(&mut stream) {
                        if !limiter.check(&username) {
                            websockets::send_frame(&mut stream, "Rate limit exceeded");
                            continue;
                        }
                        println!("Received WS message: {}", msg);

                        websockets::send_frame(&mut stream, &msg);
                    } else {
                        println!("WebSocket closed");
                        break;
                    }
                }
            }
            return;
        }
    }

    // If HTTP req

    let response = router.handle(req);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut pool = ThreadPool::new(4);
    let mut router = Router::new();
    
    router.add_route("GET","/hello",hello_handler);
    router.add_route("GET","/",home_handler);
    router.add_route("POST","/signup",signup_handler);
    router.add_route("POST","/login",login_handler);

    println!("Server launched to 127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let router = router.clone();
        pool.execute(move || {
            handle_connection(stream, &router);
        });
    }
}
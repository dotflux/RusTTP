use crate::http::HttpRequest;

pub fn hello_handler(_req:HttpRequest) -> String {
    "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello".to_string()
}

pub fn home_handler(_req:HttpRequest) -> String {
    "HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nWelcome".to_string()
}
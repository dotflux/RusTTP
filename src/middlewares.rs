use std::fs::read_to_string;
use crate::http::HttpRequest;
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Clone)]
pub struct Details {
    pub username:String,
    pub password:String
}

pub fn require_auth(_req: &HttpRequest) -> Result<String, String> {
    let session = match _req.get_cookie("user") {
        Some(s) => s,
        None => return Err("HTTP/1.1 402 BAD REQUEST\r\nContent-Length: 11\r\n\r\nBad Request".to_string()),
    };

    let contents = read_to_string("users.json").unwrap_or("[]".to_string());
    let users: Vec<Details> = serde_json::from_str(&contents).unwrap_or_default();

    if users.iter().any(|u| u.username == session) {
        Ok(session)
    } else {
        Err("HTTP/1.1 409 CONFLICT\r\nContent-Length: 15\r\n\r\nNo such user!".to_string())
    }
}

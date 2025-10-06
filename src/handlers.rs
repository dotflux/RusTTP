use crate::http::HttpRequest;
use serde::{Deserialize,Serialize};
use serde_json::from_str;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use crate::middlewares::{require_auth,Details};

pub fn hello_handler(_req:HttpRequest) -> String {
    "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello".to_string()
}

pub fn signup_handler(_req:HttpRequest) -> String {
    let body = &_req.body;

    let parsed:Result<Details,_> = from_str(body.trim());

    let signup = match parsed {
        Ok(data) => data,
        Err(_) => {
            return "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 11\r\n\r\nBad Request".to_string();
        }
    };

    let contents = read_to_string("users.json").unwrap_or("[]".to_string());

    let mut users: Vec<Details> = serde_json::from_str(&contents).unwrap_or_default();

    let user = users.iter().find(|u| u.username == signup.username);

    let user = match user {
    Some(u) => return "HTTP/1.1 409 CONFLICT\r\nContent-Length: 15\r\n\r\nUser exists!".to_string(),
    None => {
        users.push(signup);

        let new_json = serde_json::to_string_pretty(&users).unwrap();
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("users.json").unwrap();
        file.write_all(new_json.as_bytes()).unwrap();

        return "HTTP/1.1 201 CREATED\r\nContent-Length: 9\r\n\r\nSigned up".to_string();
    } ,
    };
    
}

pub fn login_handler(_req:HttpRequest)-> String {
    let body = &_req.body;

    let parsed:Result<Details,_> = from_str(body.trim());

    let login = match parsed {
        Ok(data) => data,
        Err(_) => {
            return "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 11\r\n\r\nBad Request".to_string();
        }
    };

    match _req.get_cookie("user") {
        Some(session) => {
            let body = "Already Logged In";
            return format!("HTTP/1.1 409 CONFLICT\r\nContent-Length: {}\r\n\r\n{}",body.len(),body);
        },
        None => {
            let contents = read_to_string("users.json").unwrap_or("[]".to_string());

        let users: Vec<Details> = serde_json::from_str(&contents).unwrap_or_default();

        let user = users.iter().find(|u| u.username == login.username);

        let user = match user {
        Some(u) => u,
        None => return "HTTP/1.1 409 CONFLICT\r\nContent-Length: 15\r\n\r\nNo such user!".to_string(),
        };


        if user.password != login.password {
            return "HTTP/1.1 401 UNAUTHORIZED\r\nContent-Length: 14\r\n\r\nWrong Password".to_string();
        }

        let body = "Success";
        let cookie_value = format!("user={}; HttpOnly; Path=/; Max-Age=3600", login.username);
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
            Set-Cookie: {}\r\n\
            Content-Length: {}\r\n\
            Content-Type: text/plain\r\n\
            \r\n\
            {}",
            cookie_value,
            body.len(),
            body
        );

        return response;
    }
    }
}

pub fn home_handler(_req:HttpRequest) -> String {
    let username = match require_auth(&_req) {
        Ok(u) => u,
        Err(resp) => return resp,
    };

    let body = format!("Welcome, {}",username);
    
    format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",body.len(),body)
}
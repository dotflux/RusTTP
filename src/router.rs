use std::collections::HashMap;
use crate::http::HttpRequest;

type Handler = fn(HttpRequest) -> String;

#[derive(Clone)]
pub struct Router {
    routes:HashMap<(String,String),Handler>,
}

impl Router{
    pub fn new() -> Router {
        Router {routes:HashMap::new()}
    }

    pub fn add_route(&mut self,method:&str,path:&str,handler:Handler){
        self.routes.insert((method.to_string(), path.to_string()), handler);
    }

    pub fn handle(&self,req:HttpRequest) -> String {
        if let Some(handler) = self.routes.get(&(req.method.clone(),req.path.clone())){
            handler(req)
        } else {
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
        }
    }
}
use std::collections::HashMap;

#[derive(Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn get_cookie(&self, name: &str) -> Option<String> {
        self.headers.get("Cookie").and_then(|cookies| {
            cookies.split(';')
                   .map(|c| c.trim())
                   .filter_map(|kv| {
                       let mut parts = kv.splitn(2, '=');
                       let k = parts.next()?;
                       let v = parts.next()?;
                       if k == name { Some(v.to_string()) } else { None }
                   })
                   .next()
        })
    }
}
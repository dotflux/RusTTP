use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

pub struct RateLimiter {
    pub limits:Mutex<HashMap<String,(Instant,u32)>>,
    pub max_requests:u32,
    pub window:Duration,
}

impl RateLimiter {
    pub fn new(max_requests:u32,window_secs:u64) -> Self {
        Self {
            limits:Mutex::new(HashMap::new()),
            max_requests,
            window:Duration::from_secs(window_secs),
        }
    }

    pub fn check(&self,user:&str)->bool{
        let mut limits = self.limits.lock().unwrap();
        let now = Instant::now();

        let entry = limits.entry(user.to_string()).or_insert((now,0));
        let elapsed = now.duration_since(entry.0);

        if elapsed > self.window {
            *entry = (now,1); // Reset
            true
        } else {
            if entry.1 < self.max_requests {
                entry.1 += 1;
                true
            }
            else{
                false
            }
        }

    }
}
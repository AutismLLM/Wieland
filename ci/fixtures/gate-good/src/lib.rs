use std::f64::consts::PI;
use std::io;
use std::sync::Mutex;

pub fn read_config(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

pub async fn touch(m: &Mutex<i32>) {
    let value = {
        let guard = m.lock().unwrap_or_else(|e| e.into_inner());
        *guard
    };
    yield_now().await;
    let _ = value;
}

async fn yield_now() {}

pub fn circle_area(r: f64) -> f64 {
    PI * r * r
}

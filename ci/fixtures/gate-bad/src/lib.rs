use std::sync::Mutex;

pub fn read_config(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub async fn touch(m: &Mutex<i32>) {
    let guard = m.lock().expect("poisoned");
    yield_now().await;
    let _ = *guard;
}

async fn yield_now() {}

pub fn dodgy_math() -> f64 {
    let pi = 3.14159;
    let _always = pi == pi;
    pi
}

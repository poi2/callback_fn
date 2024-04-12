use callback_rs::*;

#[before_callback(println!("Before: {:?}", chrono::Local::now()))]
#[after_callback(println!("After: {:?}", chrono::Local::now()))]
fn hello(str: &str) {
    println!("Hello {}", str)
}

// This function will print:
// Before: 2024-04-01T00:00:00.000000+09:00
// Hello world
// After: 2024-04-01T00:00:000.000200+09:00
fn main() {
    hello("world");
}

use std::{thread, time};

mod random;

fn main() {
    println!("Hello, world!");
    println!("Your random number is {}", random::random());
    thread::sleep(time::Duration::MAX);
}

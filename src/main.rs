use std::{thread, time};

mod random;

fn main() {
    println!("Hello, world!");
    println!("Your random number is {}", random::random());
    thread::sleep(time::Duration::MAX);
}

#[cfg(test)]
mod tests {
    use crate::random;

    #[test]
    fn random() -> () {
        assert_eq!(random::random(), 9);
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;
use std::io;

pub fn rng_range() {
    let mut a: String = String::new();
    let mut b: String = String::new();
    println!(
        "Whats the range we're dealing with here?\n\
        Enter the the low and high limits separately as integers followed by an \"enter\".\n\
        All values are inclusive."
    );
    io::stdin()
        .read_line(&mut a)
        .expect("Why must you constantly disappoint me?");
    io::stdin()
        .read_line(&mut b)
        .expect("You've always been a failure");
    let x: i32 = a.trim().parse().expect("Not an integer you dingus");
    let y: i32 = b.trim().parse().expect("Not an integer you dope");
    let z: i32 = rand::thread_rng().gen_range(x..=y);
    println!("The mystical prophecy foretells: {}", z);
}

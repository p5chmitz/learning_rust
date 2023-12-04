use rand::Rng;
use std::cmp::*;
use std::io::{self, Write};
//use std::{io, io::Write, cmp::Ordering};
//use std::io;
//use std::io::Write;

pub fn guessing_game() {
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Enter guess: ");
        io::stdout().flush().unwrap();
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failure. You're a failure.");
        //Shadows String guess and coverts to i32
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Guess must be an integer between 1-100");
                continue;
            }
        };
        if guess <= 0 || guess >= 100 {
            println!("Out of bounds, must be between 1-100");
            continue;
        }
        //guess and secret_number must be of the same type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too smol", guess),
            Ordering::Greater => println!("{} is too larg", guess),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}

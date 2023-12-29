#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;
use std::cmp::*;
use std::io::{self, Write};
//use std::{io, io::Write, cmp::Ordering};
//use std::io;
//use std::io::Write;

/**The book's recommended way (with some additions)*/
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
        //Shadows String guess and coverts to i32 with parse();
        //parse() returns Result which is handled with a match mechanism
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Guess must be an integer between 1-100");
                continue;
            }
        };
        //println!("{}", &secret_number);
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

/**My super hacky refactor attempt; This code adds a range validator and if statements instead of pattern matching with the compare method*/
pub fn guessing_game_2() {
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Enter guess: ");
        io::stdout().flush().unwrap();
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failure. You're a failure.");

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

        if guess < secret_number {
            println!("{} is too small", guess);
        } else if guess > secret_number {
            println!("{} is too big", guess);
        } else if guess == secret_number {
            println!("Thats it! {} is the secret number!", secret_number);
            break;
        }
    }
}

/**Validates guess with custom type which can reduce tedium of checking range for every instance
 * this logic is required; Program panicks if the range is not an integer between 1-100*/
pub fn guessing_game_3() {
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Enter guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failure. You're a failure.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Guess must be an integer.");
                continue;
            }
        };
        Guess::new(guess);

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

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value)
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

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
        //Guess::range_check(guess);

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

pub fn guessing_game_4() {
    println!("Guess the integer!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Try me: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut _val: (bool, i32) = (true, 0);
        _val = Guess::validate(&input);
        if _val.0 == true {
            _val = Guess::range_check(_val.0, _val.1);
        };
        if _val.0 == true {
            if Guess::compare(_val.1, secret_number) == true {
                break;
            }
        }
    }
}

/** Struct */
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn validate(v: &str) -> (bool, i32) {
        let mut b = true;
        let num: i32 = match v.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Does {:?} look like an integer to you?", v.trim());
                b = false;
                0
            }
        };
        (b, num)
    }
    pub fn range_check(b: bool, value: i32) -> (bool, i32) {
        let mut b = b;
        if value < 1 || value > 100 {
            println!("Number must be between 1 and 100, got {}", value);
            b = false;
        }
        (b, value)
    }
    pub fn compare(v: i32, n: i32) -> bool {
        let mut b: bool = false;
        let result = String::new();
        match v.cmp(&n) {
            Ordering::Less => println!("{} is too smol", v),
            Ordering::Greater => println!("{} is too larg", v),
            Ordering::Equal => {
                println!("You guessed it!");
                b = true;
            }
        }
        return b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guessing_game_validations() {
        let value = String::from("Peter");
        let fn_rtn = Guess::validate(&value);
        assert_eq!(fn_rtn, (false, 0));
    }
    #[test]
    fn guessing_game_range_check() {
        let fn_rtn = Guess::range_check(true, -1);
        assert_eq!(fn_rtn, (false, -1));
        let fn_rtn = Guess::range_check(true, 101);
        assert_eq!(fn_rtn, (false, 101));
        let fn_rtn = Guess::range_check(true, -0);
        assert_eq!(fn_rtn, (false, -0));
        let fn_rtn = Guess::range_check(true, 23);
        assert_eq!(fn_rtn, (true, 23));
    }
    #[test]
    fn guessing_game_compare() {
        let fn_rtn = Guess::compare(23, 23);
        assert_eq!(fn_rtn, (true));
        let fn_rtn = Guess::compare(-0, -0);
        assert_eq!(fn_rtn, (true));
    }
}

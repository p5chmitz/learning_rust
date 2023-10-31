use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Enter guess: ");
        io::stdout().flush().unwrap();
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failure. You're a failure.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Thats not a number, try again.");
                continue;
            }
        };
        //guess and secret_number must be of the same type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too smol", x),
            Ordering::Greater => println!("{} is too larg", x),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}

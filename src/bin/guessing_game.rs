use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter guess: ");
        let mut guess = String::new();
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
        match guess.cmp(&secret_number) {
            //guess and secret_number must be of the same type
            Ordering::Less => println!("Too smol"),
            Ordering::Greater => println!("Too larg"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}

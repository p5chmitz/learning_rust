use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Enter number: ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failure. You're a failure.");
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too smol"),
        Ordering::Greater => println!("Too larg"),
        Ordering::Equal => println!("You guessed it!")
    }
}

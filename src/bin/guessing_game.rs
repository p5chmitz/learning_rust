use std::io;

fn main() {
    println!("Guess the number!");
    println!("Enter number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failure. You're a failure.");
    println!("You guessed: {guess}");
}

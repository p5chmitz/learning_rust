use std::io;
use std::str;

//Instantiates a String class and binds user input to the variable
fn new_string_class() -> String {
    let mut a: String = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failure. You're a failure.");
    return a;
}
//Instantiates a String class and binds hardcoded literal to the variable
fn new_string_class_literal() -> String {
    let mut a: String = String::from("hello");
    return a;
}
//Creates a new reference and binds hardcoded literal to the variable
fn new_string_literal() {
    let a: &str = "henlo!";
    println!("{a}");
}

fn main() {
    println!("{:?}", new_string_class());
    println!("{}", new_string_class_literal());
    new_string_literal();
}
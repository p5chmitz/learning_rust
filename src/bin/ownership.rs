use std::io;

//Instantiates a String class and binds user input to the variable
fn new_string_class() {
    let mut a: String = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failure. You're a failure.");
}
//Instantiates a String class and binds hardcoded literal to the variable
fn new_string_class_literal() {
    let mut b: String = String::from("hello");
}
//Creates a new reference and binds hardcoded literal to the variable
fn new_string_literal() {
    let a: &str = "hello!";
}

fn main() {

}
//use std::io;
//use std::str;

//Instantiates a String class and binds user input to the variable
fn new_string_class() -> String {
    let mut a: String = String::new();
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failure. You're a failure.");
    return a;
}
//Instantiates a String class and binds hardcoded literal to the variable
fn new_string_class_literal() -> String {
    let mut a: String = String::from("Hello");
    a.push_str(" world!");
    return a;
}
//Creates a new reference and binds hardcoded literal to the variable
fn new_string_literal() {
    let a: &str = "henlo!";
    println!("{a}");
}

//Shallow copies
fn shallow_copy() {
    let a: i32 = 12;
    let b: i32 = a;
    println!("a = {}, and b = {}.", a, b);
}
//Deep copy
fn deep_fucking_copy() {
    let x: String = String::from("Peter");
    let mut y: String = x.clone();
    y.push_str(" Schmitz");
    println!("x = {}, and y = {}.", x, y);
}

fn main() {
    println!("{:?}", new_string_class());
    println!("{}", new_string_class_literal());
    new_string_literal();
    shallow_copy();
    deep_fucking_copy();
}
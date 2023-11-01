//use std::io;
//use std::str;

use std::io::Write;

//Instantiates a String class and binds user input to the variable
fn new_string_class() -> String {
    let mut a: String = String::new();
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failure. You're a failure.");
    return a;
}
fn new_string_class_contained() {
    let mut a: String = String::new();
    print!("Enter phrase: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failure. You're a failure.");
    print!("function output: {}", a);
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

//Shallow copies with
fn shallow_copy() {
    let a: i32 = 12;
    let b: i32 = a;
    println!("a = {}, and b = {}.", a, b);
}
//"Deep copies" using clone
fn deep_fucking_copy() {
    let x: String = String::from("Peter");
    let mut y: String = x.clone();
    y.push_str(" Schmitz");
    println!("x = {}, and y = {}.", x, y);
}

fn var_scope_complex(i: String) -> String {
    let mut a: String = i;
    a.push_str(" world");
    return a;
}
fn var_scope_primitive(i: i32) -> i32 {
    let a: i32 = i + 10;
    return a;
}

fn main() {
    //This design uses a function that returns a value
    print!("Enter phrase: ");
    std::io::stdout().flush().unwrap();
    let a: String = new_string_class();
    print!("main() output: {}", a);
    //This design does the same thing but with a self-contained void function
    new_string_class_contained();

    println!("{}", new_string_class_literal());
    new_string_literal();
    shallow_copy();
    deep_fucking_copy();

    //Requires a clone() because the heap-allocated type exits scope at its first use
    let s: String = String::from("Hello");
    let cloned_var: String = s.clone();
    println!("Modified greeting: {}", var_scope_complex(cloned_var));
    println!("Original greeting: {s}");

    //Implicit stack frame copy is produced so the variable never leaves scope
    let n: i32 = 30;
    println!("Modified number: {}", var_scope_primitive(n));
    println!("Original number: {n}");

}

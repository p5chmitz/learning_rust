#![allow(dead_code)]
#![allow(unused_variables)]

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
    let _a: &str = "henlo!";
    dbg!(_a);
    //println!("{a}");
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
pub fn move_it() {
    let a: i32 = 32;
    let b: i32 = a;
    println!("{a}");
    let ass: String = String::from("I like to move it, move it");
    let mut bass = ass.clone();
    bass.push_str("!"); //requires clone because of push_str() move
    println!("{ass}");
    println!("{bass}");
}

//References
fn calc_len(s: &String) -> usize {
    //The function takes a reference type
    let i: usize = s.len();
    return i;
}
pub fn ownership_1() {
    let mut v = vec![1, 2, 3];
    //let a = &v[2]; //Immutable borrow
    v.push(4); //Mutable borrow causes immutable borrow to get deallocated
    let a = &v[2]; //Immutable borrow
    println!("{}", a); //Requires either shadowed allocation or mutable borrow, otherwise immutable borrow is out-of-scope

    let mut s = String::from("Peter");
    //let a = &s; //Immutable borrow
    s.push_str("!"); //Mutable borrow
    let a = &s; //Immutable borrow
    println!("{a}"); //Illegal?
}
pub fn ownership_2() {
    let s: String = String::from("Hello");
    let cloned_var: String = s.clone();
    println!("Modified greeting: {}", ownership_2a(cloned_var));
    println!("Original greeting: {}", s);
}
fn ownership_2a(i: String) -> String {
    let mut a: String = i;
    a.push_str(" world");
    return a;
}
pub fn ownership_3() {
    let s = String::from("Peter");
    let s1 = ownership_3a(s);
    println!("{}", s1);
}
fn ownership_3a(s: String) -> String {
    s
}


fn main() {
    //This design uses a function that returns a value
    print!("Enter phrase: ");
    std::io::stdout().flush().unwrap();
    let a: String = new_string_class();
    print!("main() output: {}", a);
    //This design does the same thing but with a self-contained void function
    new_string_class_contained();

    //Runs the New String class literal function that appends "world" to hello
    println!("{}", new_string_class_literal());

    //Binds and prints a slice
    new_string_literal();

    //Copy-type functions
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

    //References
    //Rust would otherwise move s1 when we pass it to the calling function.
    //In order to use s1 after passing it to a function we can either create a tuple to return it,
    //or we can create a reference to the value
    let s1: String = String::from("Peter");
    let _s2: usize = calc_len(&s1); //Create a reference type
    println!("The string {s1} is {_s2} characters long.");
}

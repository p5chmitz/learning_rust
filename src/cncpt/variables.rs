#![allow(dead_code)]
#![allow(unused_variables)]

//Basic declaration, instantiation, and printing
fn basic_declaration() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The NEW value of x is: {}", x);
}
//Declaring and using constants
fn constants() {
    const PIE: f32 = 3.1459;
    const TWO_PIE: f32 = PIE * 2.0;
    let mon: i32 = 12;
    let yr: i32 = 23;
    println!("\nCombining {} with {} results in {}.", mon, yr, (mon + yr));
    println!(
        "The value of pie is roughly {}, and double that is roughly {}.",
        PIE, TWO_PIE
    );
}
//Example of (over)shadowing where multiple variables are created
//with the same name, but hold different values .
fn shadow_one() {
    let n: u32 = 3;
    println!("\nWhat is n?! Easy, its {}.", n);
    {
        let n: i32 = 32;
        println!("What is n in the new scope?! Easy, its {}.", n);
    }
    println!("What is n?! Easy, its {}.", n);
    let n: f32 = 32.3;
    println!("What is n?! Easy, its {}.", n);
}
//Example of shadowing that allows us to change type
fn shadow_two() {
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("\nThe value of spaces is \"{}\".", spaces);
}
//This example accomplishes the same thing as above, but does not use shadowing
fn shadow_three() {
    let phrase: String = String::from("Whatever we want it to, now");
    //phrase = "Whatever we want it to, now".parse().expect("lol");
    println!("\nThe value of spaces is \"{}\".", phrase.len());
    println!("{}", phrase);
}
//Integer overflow illustration with wrapping method
fn integer_overflow() {
    let mut num: u8 = 255;
    println!("Num is {num}");
    num = num.wrapping_add(2);
    //num += 1;
    println!("If we add 2 with a wrapping method, num becomes {num}");
}
//Integer rounding
fn integer_rounding() {
    let a: i32 = 24;
    let b: i32 = 5;
    let _c: i32 = a / b;
    println!("{a}/{b} = {_c}");
}

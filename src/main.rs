#![allow(dead_code)]
#![allow(unused_variables)]
//#![allow(unused_imports)]

//#![allow(dead_code)]
//Sub-modules/funcitons re-exported for cleaner access to functions.
//See top-level modules for export and path details
//cncpt::loops::_my_age_static();

use cncpt::generics::Tweet;
//use cncpt::types::smart_pointers::Cons::{Cons, Nil};
use std::io;

use crate::cncpt::error_handling::{error_handling_6, error_handling_8};
//use std::io::{self, Write};

mod cncpt;
mod exmpl;
mod sandbox;
mod util;

/**Prints a formatted timestamp*/
fn timestamp(ver: i32) {
    let mut prompt: String = String::new();
    let mut c1: char = ' ';
    let mut c2: char = ' ';
    let mut s: String = String::new();
    match ver {
        1 => {
            prompt.push_str("Start time: ");
            c1 = '=';
            c2 = ' ';
        }
        2 => {
            prompt.push_str("End time: ");
            c1 = ' ';
            c2 = '=';
        }
        _ => {
            println!("Error!")
        }
    }
    s.push_str(&prompt);
    s.push_str(&util::time::static_time(8));
    print_divider(s.len(), c1);
    println!("\n{}{}", prompt, util::time::static_time(8));
    print_divider(s.len(), c2);
    println!("")
}
fn print_divider(len: usize, c: char) {
    let mut l = 0;
    while l < len {
        print!("{}", c);
        l += 1;
    }
}

fn main() {
    timestamp(1);
    println!(" ");
    loop {
        println!(
            "============\nPROGRAM MENU\n============\n\
            1: Guessing game (book)\n\
            2: Guessing game (my hacky bullshit)\n\
            3: Clock (loop)\n\
            4: Vector analyzer\n\
            5: Pig Latin translator\n\
            6: Exit (rando function tests)"
        );
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failure. You're a failure.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nERROR: Enter a number\n");
                continue;
            }
        };
        if input < 1 || input > 6 {
            println!("\nERROR: Enter a valid menu option number\n");
            continue;
        }
        match input {
            1 => exmpl::guessing_game(),
            2 => exmpl::guessing_game::guessing_game_5(),
            3 => util::time::loop_time(8),
            4 => cncpt::collections::book_test_1(),
            5 => cncpt::collections::book_test_2(),
            _ => {
                println!("\n[EXIT]\n");
                break;
            }
        }
    }

    //cncpt::test_framework::function_tests();
    cncpt::test_framework::function_tests_1();

    // Closures
    let mut s: String = String::new();
    s.push_str("lol");
    let rn = cncpt::closures::printinator();
    println!("{}", cncpt::closures::closures_1(&s).unwrap_or_else(|| &rn));

    // Error handling
    let file = match error_handling_6() {
        Ok(file) => file,
        Err(e) => format!("Error: {}", e),
    };
    println!("File contents: {}", file);

    let file = error_handling_6().unwrap_or_else(|e| format!("Error handling 6: {}", e));
    println!("File contents: {}", file);

    error_handling_8().unwrap_or_else(|e| eprintln!("Error handling 8: {}", e));
    if let Ok(()) = error_handling_8() {
        println!("Its all good")
    } else {
        println!("We have terrible news about your file...")
    };

    cncpt::error_handling::error_handling_11();
    cncpt::error_handling::error_handling_12();

    //match cncpt::error_handling::error_handling_8() {
    //    Ok(()) => println!("It works!"),
    //    Err(ref e) => eprintln!("Error: {}", e),
    //};

    //if let Ok(()) = cncpt::error_handling::error_handling_8() {
    //    println!("Success!")
    //} else {
    //    println!("An error occurred...")
    //}

    let file = std::fs::read_to_string("./src/cncpt/hello.txt").unwrap();
    println!("Read from main: {}", file);

    // SMART POINTERS
    cncpt::types::smart_pointers::smart_pointers_1();
    //let list = cncpt::types::smart_pointers::Cons(1, (2, (3, Nil)));
    //let list = cncpt::types::smart_pointers::Cons::Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //println!("The cons list: {:#?}", list);
    println!("");

    let index_val: usize = 1000000;
    let mut pred: u128 = 0;
    let mut curr: u128 = 1;
    for _ in 1..index_val {
        let next = curr.wrapping_add(pred);
        pred = curr;
        curr = next;
    }
    println!("The Fibonacci number at index {} is {}", index_val, curr);

    let i = 100;
    let mut n: u128 = 1;
    let mut p: u128 = 0;
    let mut c = 1;
    while c < i {
        n = p + n;
        p = n - p;
        c += 1;
    }
    println!("The fib num at {i} is {n}");

    cncpt::types::smart_pointers::ref_counter();
    exmpl::json::parsing_json_1();

    exmpl::json::idk_anymore();

    timestamp(2);
}

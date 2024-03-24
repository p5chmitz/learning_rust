//Sub-modules/funcitons re-exported for cleaner access to functions.
//See top-level modules for export and path details
//cncpt::loops::_my_age_static();

use cncpt::generics::Tweet;
use std::io;
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
            2 => exmpl::guessing_game::guessing_game_4(),
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

    let mut s: String = String::new();
    s.push_str("lol");
    let rn = cncpt::closures::printinator();
    println!("{}", cncpt::closures::closures_1(&s).unwrap_or_else(|| &rn));

    //cncpt::io::_io_3();
    let file = match cncpt::error_handling::error_handling_6() {
        Ok(file) => file,
        Err(e) => format!("{}", e),
    };
    println!("File contents: {}", file);
    let file = cncpt::error_handling::error_handling_6().unwrap_or_else(|e| format!("Error: {}", e));
    println!("File contents: {}", file);
    cncpt::error_handling::error_handling_11();
    cncpt::error_handling::error_handling_12();


    timestamp(2);
}

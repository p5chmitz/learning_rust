//Sub-modules/funcitons re-exported for cleaner access to functions.
//See top-level modules for export and path details
//cncpt::loops::_my_age_static();

use std::io;
use cncpt::traits::{Tweet, NewsArticle};

mod cncpt;
mod exmpl;
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
        _ => {}
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
            2 => exmpl::guessing_game_2(),
            3 => util::time::loop_time(8),
            4 => cncpt::collections::book_test_1(),
            5 => cncpt::collections::book_test_2(),
            _ => {
                println!("\n[EXIT]\n");
                break;
            }
        }
    }

    //cncpt::error_handling::error_handling_1();
    //cncpt::error_handling::error_handling_2();
    //exmpl::test::calculate(1, 8);
    cncpt::generics::generics_1();
    cncpt::traits::traits_1();
    let tweet = Tweet {
        username: String::from("pschmitz"),
        content: String::from("Twitter is mostly just depressed millenial jokes"),
        reply: false,
        retweet: true,
    };

    cncpt::traits::traits_2(&tweet);
    cncpt::traits::traits_3(&tweet);
    let s = String::from("hello, world!");
    let s1 = 23;
    cncpt::traits::traits_4(&s);
    cncpt::traits::traits_4(&s1);

    timestamp(2);
}

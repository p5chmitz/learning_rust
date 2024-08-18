// These two life savers suppress warnings throughout the codebase during compilation
// For production remove these to get rid of excess or unnecessary code
#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;

use cncpt::generics::Tweet;
use util::main_utils::timestamp;
//use cncpt::types::smart_pointers::Cons::{Cons, Nil};
use crate::cncpt::error_handling::{error_handling_6, error_handling_8};
//use std::io::{self, Write};

mod cncpt;
mod exmpl;
mod sandbox;
mod util;

fn as_byte_array() {
    let w = "Peter".to_string();
    println!("{:?}", w.as_bytes());
    assert_eq!(65, "A".as_bytes()[0]);
    assert_eq!(65, 0x41);
    assert_eq!(65, 'A' as i32);
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
            6: Random data conversion routines\n\
            7: Closures test\n\
            8: Exit (rando function tests)"
        );
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Seriously, how did you fuck up this bad?");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nERROR: Enter a number\n");
                continue;
            }
        };
        if input < 1 || input > 8 {
            println!("\nERROR: Enter a valid menu option number\n");
            continue;
        }
        match input {
            1 => exmpl::guessing_game(),
            2 => exmpl::guessing_game::guessing_game_5(),
            3 => util::time::loop_time(8),
            4 => cncpt::collections::book_test_1(),
            5 => cncpt::collections::book_test_2(),
            6 => sandbox::data_conversions::data_conversion_driver(),
            7 => cncpt::closures::closures_driver(),
            _ => {
                println!("\n[EXIT]\n");
                break;
            }
        }
    }

    ////////////////////
    // ✨ EXAMPLES ✨ //
    ////////////////////

    // PRIMITIVE TYPES
    cncpt::types::primitives::reference::slices_2();
    cncpt::types::primitives::reference::references_3();
    cncpt::types::primitives::reference::slices_0();

    cncpt::types::types::dst();

    // ADVANCED TYPES
    cncpt::types::types::using_meter();

    // CONTROL FLOW
    cncpt::ctrl_flow::match_like::idk_anymore();
    cncpt::ctrl_flow::match_like::calling_function();
    cncpt::ctrl_flow::if_let::if_let_1(3);
    cncpt::ctrl_flow::loops::for_loops_2();

    // ITERATORS
    cncpt::iterators::iter_test();

    print!("TESTING");

    // ERROR HANDLING
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

    let file =
        std::fs::read_to_string("./src/cncpt/hello.txt").unwrap_or_else(|e| format!("Fuck: {}", e));
    println!("Read from main: {}", file);

    // TRAITS
    cncpt::traits::calling_code();
    cncpt::traits::calling_code_2();

    // DATA STRUCTURES
    cncpt::types::compound::collections::array_7();
    exmpl::data_structures::linked_lists::singly_linked_list();
    exmpl::data_structures::linked_lists::rectangle();
    as_byte_array();

    // SMART POINTERS
    cncpt::types::smart_pointers::smart_pointers_1();
    //let list = cncpt::types::smart_pointers::Cons(1, (2, (3, Nil)));
    //let list = cncpt::types::smart_pointers::Cons::Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //println!("The cons list: {:#?}", list);
    println!("");

    cncpt::types::smart_pointers::ref_counter();

    // SERIALIZED DATA
    exmpl::json::parsing_json_1();

    // CONCURRENCY
    cncpt::concurrency::concurrency_5();

    // UNSAFE RUST
    cncpt::unsafe_rust::unsafe_1();
    cncpt::unsafe_rust::unsafe_6();

    timestamp(2);
}

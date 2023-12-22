//Sub-modules/funcitons re-exported for cleaner access to functions.
//See top-level modules for export and path details
//cncpt::loops::_my_age_static();

use std::io;

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
    //cncpt::ownership::ownership_1();
    //cncpt::ownership::ownership_2();
    //cncpt::ownership::ownership_3();
    //cncpt::structs::struct_demo_1();
    //cncpt::strings::string_slice_4();
    //cncpt::strings::string_slice_5();
    //cncpt::strings::string_wrapper_1();
    //cncpt::strings::string_wrapper_2();
    //cncpt::strings::string_wrapper_2();
    //cncpt::strings::string_wrapper_3();
    //cncpt::collections::hash_maps_1();
    //cncpt::collections::hash_maps_2();
    //cncpt::collections::hash_maps_3();
    //cncpt::collections::book_test_1();
    //cncpt::collections::book_test_2();
    //cncpt::collections::vec_test_6(10);
    //cncpt::collections::vec_test_2(23);
    //cncpt::collections::vec_test_5(10);
    //cncpt::error_handling::error_handling_1();
    cncpt::io::io_1();
    cncpt::io::io_2();
    //exmpl::json::json_parsing();
    //cncpt::collections::vec_test_3();
    timestamp(2);
}

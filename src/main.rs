use std::io;

mod cncpt;
mod exmpl;
mod util;

fn timestamp(g: i32) {
    let mut s: String = String::new();
    let mut prompt: String = String::new();
    match g {
        1 => {
            prompt.push_str("Start time: ");
        },
        2 => {
            prompt.push_str("End time: ");
        },
        _ => {}
    }
    s.push_str(&prompt);
    s.push_str(&util::time::static_time(8));
    let mut l = 0;
    while l < s.len() {
        print!("=");
        l += 1;
    }
    println!("\n{}{}", prompt, util::time::static_time(8));
    let mut l = 0;
    while l < s.len() {
        print!("=");
        l += 1;
    }
    println!("")
}

fn main() {
    //println!("Im a root crate");
    //println!("Start time: {}", util::time::static_time(8));
    timestamp(1);
    loop {
        //println!("=====================\n");
        println!(
            "PROGRAM MENU\n\
            1: Guessing game (book)\n\
            2: Guessing game (my hacky bullshit)\n\
            3: Clock (loop)\n\
            4: Exit (rando function tests)"
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
        if input < 1 || input > 4 {
            println!("\nERROR: Enter a valid menu option number\n");
            continue;
        }
        println!("\n=====================\n");
        match input {
            1 => exmpl::guessing_game(),
            2 => exmpl::guessing_game_2(),
            3 => util::time::loop_time(8),
            //4 => break,
            _ => break,
        }
    }

    //Sub-modules/funcitons re-exported for cleaner access to functions.
    //See top-level modules for export and path details
    //cncpt::loops::_my_age_static();
    cncpt::collections::vec_test_2(10);
    exmpl::json::json_parsing();

    timestamp(2);
}

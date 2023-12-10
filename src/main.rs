use std::io;

mod cncpt;
mod exmpl;
mod util;

fn main() {
    //println!("Im a root crate");
    println!("Start time: {}", util::time::static_time(8));
    loop {
        println!(
            "PROGRAM MENU\n\
            1: Guessing game (book)\n\
            2: Guessing game (my hacky bullshit)\n\
            3: Clock (loop)"
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
        if input < 1 || input > 3 {
            println!("\nERROR: Enter a valid menu option number\n");
            continue;
        }
        match input {
            1 => exmpl::guessing_game(),
            2 => exmpl::guessing_game_2(),
            3 => util::time::loop_time(8),
            _ => break,
        }
    }

    cncpt::ctrl_flow::loops::_my_age_static();
    //Sub-module/funciton re-exported in exmpl.rs/util.rs for cleaner access to example programs
    cncpt::vectors::vec_test_1();
    cncpt::vectors::vec_test_2(10);

    //Guessing games
    exmpl::guessing_game();
    //exmpl::guessing_game_2();
    //exmpl::rng_range();

    cncpt::ctrl_flow::ifs::if_statements(0.45);
    cncpt::ctrl_flow::ifs::again_lets_if(6);
    util::time::loop_time(8);
}

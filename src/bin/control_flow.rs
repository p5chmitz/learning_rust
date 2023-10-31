//If statements are the basic building block of control flow
//Takes a float64 and prints the range
fn if_statements(n: f64) {
    if n < 0.33 {
        println!("lower third")
    } else if (0.33 < n) && (n < 0.66) {
        println!("middle third")
    } else if 0.66 < n {
        println!("upper third");
    }
}
//Takes a float64 and returns a String range
fn so_many_times(n: f64) -> String {
    let mut s: String = String::new();
    if n < 0.33 {
        s = String::from("lower third");
    } else if (0.33 < n) && (n < 0.66) {
        s = String::from("mids");
    } else if 0.66 < n {
        s = String::from("upper third");
    };
    return s;
}
//Uses an if statement in a declaration
fn again_lets_if(n: i32) {
    let i: &str = if n < 5 { "true" } else { "false" };
    println!("{i}");
}
//Introduces the basic loop keyword
fn loops() {
    let mut i: i32 = 12;
    loop {
        println!("{i}");
        i += 1;
        if i == 23 {
            println!("{i} <- nice");
            i += 1;
            continue;
        }
        if i == 30 {
            println!("The max is: {i}");
            break;
        }
    }
}
//Uses a loop in a declaration
fn loop_lets() {
    let mut i: i32 = 12;
    let x: i32 = loop {
        println!("{i}");
        i += 1;
        if i == 23 {
            println!("{i} <- nice");
            i += 1;
            continue;
        }
        if i == 30 {
            break i;
        }
    };
    println!("The max is: {x}");
}
//Uses loop labels for nested/scoped identification
fn loop_labels() {
    let mut count: i32 = 1;
    loop {
        println!("{count}");
        let mut second_count: i32 = 101;
        'second_loop: loop {
            println!("{second_count}");
            second_count += 1;
            if second_count == 110 {
                break 'second_loop;
            }
        }
        count += 1;
        if count == 4 {
            break;
        }
    }
}
//While loops, yo
fn while_loops() {
    let mut i: i32 = 10;
    while i >= 1 {
        println!("{i}");
        i -= 1;
    }
    println!("Blast off!");
}
//Playing around with while loops
fn my_age() {
    let mut age: i32 = 0;
    let mut year: i32 = 1983;
    println!("I was born in {}.", year);
    year += 1;
    while year <= 2023 {
        println!(
            "For most of {} I was {}, but at the very end I turned {}.",
            year,
            age,
            (age + 1)
        );
        age += 1;
        year += 1
    }
}
//Just a static age calculator
fn my_age_static() {
    let year: i32 = 2023;
    let mut age: i32 = year - 1984;
    println!(
        "For most of {} I was {}, but at the very end I turned {}.",
        year,
        age,
        (age + 1)
    )
}
//Just a static age calculator that takes a variable
fn my_age_again(year: i32) {
    let mut age: i32 = year - 1984;
    println!(
        "For most of {} I was {}, but at the very end I turned {}.",
        year,
        age,
        (age + 1)
    );
}
//For loop with an array
fn for_loops() {
    let a: [&str; 4] = ["a", "b", "c", "d"];
    for index in a {
        println!("{index}")
    }
    for i in (1..10).rev() {
        println!("{i}")
    }
    println!("Blorst off!");
}

fn main() {
    if_statements(0.45);
    println!("{}", so_many_times(0.5));
    again_lets_if(6);
    // loops();
    // loop_lets();
    // loop_labels();
    // while_loops();
    // my_age();
    // my_age_static();
    // my_age_again(2023);
    // for_loops();
}

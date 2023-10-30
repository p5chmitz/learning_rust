use json;
use json::{object, JsonValue};

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
//This example accomplishes the same thing, but does not use shadowing
//It appears I may not understand Strings yet
fn shadow_three() {
    let mut phrase: String = String::new();
    phrase = "Whatever we want it to, now".parse().expect("lol");
    println!("\nThe value of spaces is \"{}\".", phrase.len());
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
    let c: i32 = a / b;
    println!("{a}/{b} = {c}");
}
//Its tuples!
fn tuple() {
    //declares a tuple of mixed types
    let idk: (i32, f64, u8) = (32, 6.4, 8);
    let (a, b, c) = idk; //access all elements
    let d = idk.1; //access specific elements by index
    println!("The whole tuple: {a}, {b}, {c}");
    println!("Accessing a tuple index: {d}");

    //declares a tuple of the same type
    let whatev = (1, 2, 3, 4);
    let e = whatev.3;
    println!("When the tuple hits just right, {e}");
}
//Its an array, Carlie Brown!
fn array() {
    //Be explicit
    let array: [f64; 3] = [32.0, 6.4, 8.0];
    let a: f64 = array[0]; //access specific elements by index
    println!("Accessing a tuple index: {a}");
    //Be implicit
    let array_two = [23; 5];
    let b: i32 = array_two[3];
    println!("Lets print a hastily initialized array index: {b}");
}
//Example of a while loop.
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
//Example of a while loop.
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
//Example of a while loop.
fn my_age_again(year: i32) {
    let mut age: i32 = year - 1984;
    println!(
        "For most of {} I was {}, but at the very end I turned {}.",
        year,
        age,
        (age + 1)
    );
}
fn return_type() -> i32 {
    let x: i32 = 12;
    let y: i32 = 23;
    x + y
}
fn return_type_two() -> i32 {
    let x: i32 = 23;
    let y: i32 = 83;
    return x + y;
}
fn working_with_return_statements() {
    println!("{} | {}", return_type(), return_type_two());
}
fn if_statements(n: f64) {
    if n < 0.33 {
        println!("lower third")
    }
    else if (0.33 < n) && (n < 0.66) {
        println!("middle third")
    }
    else if 0.66 < n {
        println!("upper third");
    }
}
// fn so_many_times(n: f64) {
//     let i = {
//         if n < 0.33 {33}
//         else if (0.33 < n) && (n < 0.66) {50}
//         else if 0.66 < n {66};
//         return ()
//     };
//     println!("{i}");
// }
fn again_lets_if(n: i32) {
    let i: &str = if n < 5 {"true"} else {"false"};
    println!("{i}");
}
fn loops() {
    let mut i: i32 = 12;
    loop {
        println!("{i}");
        i += 1;
        if i == 23 {
            println!("{i} <- nice");
            i += 1;
            continue
        }
        if i == 30 {
            println!("The max is: {i}");
            break;
        }
    }
}
fn loop_lets() {
    let mut i: i32 = 12;
    let x: i32 = loop {
        println!("{i}");
        i += 1;
        if i == 23 {
            println!("{i} <- nice");
            i += 1;
            continue
        }
        if i == 30 {
            break i;
        }
    };
    println!("The max is: {x}");
}
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
fn while_loops() {
    let mut i: i32 = 10;
    while i >= 1 {     //Use the loop expression to bind a variable
        println!("{i}");
        i -= 1;
    };
    println!("Blast off!");
}
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
//JSON parsing experiment
fn json_parsing() {
    let parsed: JsonValue = json::parse(
        r#"
        {
            "key": "12/23/1983",
            "anotherKey": "value",
            "object": {
                "nestedOne": "one",
                "nestedTwo": "two"
            }
        }
        "#,
    )
    .unwrap();
    let instantiated = object! {
        "key": "12/23/1983",
        "anotherKey": "value",
        "object": {
            "nestedOne": "one",
            "nestedTwo": "two"
        }
    };
    println!("The \"key's\" value is: \"{}\"", parsed["key"]);
    println!(
        "For the parsed object, the nested key \"nestedOne\" value is: \"{}\"",
        (parsed["object"]["nestedOne"])
    );
    println!(
        "For the instantiated object, the nested key \"nestedTwo\" value is: \"{}\"",
        (instantiated["object"]["nestedTwo"])
    );
}

fn main() {
    // basic_declaration();
    // constants();
    // shadow_one();
    // shadow_two();
    // shadow_three();
    //my_age();
    //json_parsing();
    // integer_overflow();
    // integer_rounding();
    // tuple();
    // array();
    my_age_static();
    my_age_again(2023);
    working_with_return_statements();
    if_statements(0.45);
    //if_statement_in_let(0.56);
    //loops();
    //again_lets_if(6);
    // so_many_times(0.5); //doesn't actually work
    loop_lets();
    loop_labels();
    //while_loops();
    for_loops();
}

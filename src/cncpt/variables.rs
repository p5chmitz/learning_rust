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
//Its tuples!
fn tuple() {
    //declares a tuple of mixed types
    let idk: (i32, f64, u8) = (32, 6.4, 8); //Creates a tuple
    let (_a, _b, _c) = idk; //access all elements
    let _d: &f64 = &idk.1; //access specific elements by index
    println!("The whole tuple: {_a}, {_b}, {_c}");
    println!("Accessing a tuple index: {_d}");

    //Declares a User tuple
    let user: (bool, String, String, u64) = (
        true,
        String::from("Pschmitz"),
        String::from("peter@email.com"),
        23,
    );
    let _user_name = user.1;
    let _email = user.2;
    println!("Username: {_user_name}");
    println!("Email: {_email}");

    //declares a tuple of the same type
    let whatev = (1, 2, 3, 4);
    let _e = whatev.3;
    println!("When the tuple hits just right, {_e}");
}
fn tuple_two() {
    let x: (String, u8) = (String::from("Peter"), 40);
    let _name: String = String::from(x.0);
    let _age: u8 = x.1;
    println!("{_name} is {_age} years old.");
}
//Its an array, Carlie Brown!
fn array() {
    //Be explicit
    let array: [f64; 3] = [32.0, 6.4, 8.0];
    let _a: &f64 = &array[0]; //access specific elements by index
    println!("Accessing a tuple index: {_a}");
    //Be implicit
    let array_two = [23; 5];
    let _b: &i32 = &array_two[3];
    println!("Lets print a hastily initialized array index: {_b}");
}

//Str type
fn str_type() {
    let _s: &str = "Peter";
}
//String Slice (not really a type, but a reference to a String index)
fn slice_type() {
    let s = String::from("Hello, world!");
    let _hello = &s[0..5]; //References the first 5 indexes
    let _hello = &s[1..5]; //Result is same as above
    let _world = &s[7..12]; //References another 5 indexes
    println!("{_hello} || {_world}");

    let _hello_world = &s[..]; //References the whole index range
    println!("{_hello_world}");
}

//Function that returns a value
fn return_type() -> i32 {
    let x: i32 = 12;
    let y: i32 = 23;
    x + y
}
//Another function that returns a value
fn return_type_two() -> i32 {
    let x: i32 = 23;
    let y: i32 = 83;
    return x + y;
}
//Exploring return statements
fn working_with_return_statements() {
    println!("{} | {}", return_type(), return_type_two());
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
    basic_declaration();
    constants();
    shadow_one();
    shadow_two();
    shadow_three();
    json_parsing();
    integer_overflow();
    integer_rounding();
    tuple();
    tuple_two();
    str_type();
    array();
    working_with_return_statements();
    slice_type();
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}

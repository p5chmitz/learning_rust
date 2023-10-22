use json;
use json::JsonValue;

fn main() {
    let mut x:i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The NEW value of x is: {}", x);

    const PIE: f32 = 3.1459;
    const TWO_PIE: f32 = PIE * 2.0;
    let mon: i32 = 12;
    let yr: i32 = 23;
    println!("\nCombining {} with {} results in {}.", mon, yr, (mon + yr));
    println!("The value of pie is roughly {}, and double that is roughly {}.", PIE, TWO_PIE);

    let n: u32 = 3;
    println!("\nWhat is n?! Easy, its {}.", n);
    {
        let n: i32 = 32;
        println!("What is n in the new scope?! Easy, its {}.", n);
    }
    println!("What is n?! Easy, its {}.", n);
    let n: f32 = 32.3;
    println!("What is n?! Easy, its {}.", n);

    //let mut spaces: String = String::new();
    let spaces = "   ";
    let spaces = spaces.len();
    println!("\nThe value of spaces is \"{}\".", spaces);

    let mut age: i32 = 0;
    let mut year: i32 = 1983;
    println!("I was born in {}.", year);
    year += 1;
    while year <= 2023 {
        println!("For most of {} I was {}, but at the very end I turned {}.", year, age, (age + 1));
//        println!("I turned {} at the very end of {}.", age, year);
        age += 1;
        year += 1;
    }

    let parsed: JsonValue = json::parse(
        r#"
            {
                "key": "12/23/1983",
                "anotherKey": "value"
            }
        "#
        ).unwrap();
    println!("The \"key's\" value is: \"{}\"", parsed["key"]);

}
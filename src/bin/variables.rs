use json;
use json::{JsonValue, object};

fn main() {
    /** Basic declaration, instantiation, and printing */
    let mut x:i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The NEW value of x is: {}", x);

    /** Declaring and using constants */
    const PIE: f32 = 3.1459;
    const TWO_PIE: f32 = PIE * 2.0;
    let mon: i32 = 12;
    let yr: i32 = 23;
    println!("\nCombining {} with {} results in {}.", mon, yr, (mon + yr));
    println!("The value of pie is roughly {}, and double that is roughly {}.", PIE, TWO_PIE);

    /** Example of (over)shadowing where multiple variables are created
     with the same name, but hold different values .*/
    let n: u32 = 3;
    println!("\nWhat is n?! Easy, its {}.", n);
    {
        let n: i32 = 32;
        println!("What is n in the new scope?! Easy, its {}.", n);
    }
    println!("What is n?! Easy, its {}.", n);
    let n: f32 = 32.3;
    println!("What is n?! Easy, its {}.", n);

    /** Example of shadowing that allows us to change type */
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("\nThe value of spaces is \"{}\".", spaces);

    /** This example accomplishes the same thing, but does not use shadowing */
    let mut phrase: String = String::new();
    phrase = "Whatever we want it to, now".parse().expect("lol");
    println!("\nThe value of spaces is \"{}\".", phrase.len());

    /** Example of a whole loop. */
    let mut age: i32 = 0;
    let mut year: i32 = 1983;
    println!("I was born in {}.", year);
    year += 1;
    while year <= 2023 {
        println!("For most of {} I was {}, but at the very end I turned {}.", year, age, (age + 1));
        age += 1;
        year += 1;
    }

    /** Using the JSON crate */
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
        "#
        ).unwrap();
    let instantiated = object!{
        "key": "12/23/1983",
        "anotherKey": "value",
        "object": {
            "nestedOne": "one",
            "nestedTwo": "two"
        }
    };
    println!("The \"key's\" value is: \"{}\"", parsed["key"]);
    println!("For the parsed object, the nested key \"nestedOne\" value is: \"{}\"", (parsed["object"]["nestedOne"]));
    println!("For the instantiated object, the nested key \"nestedTwo\" value is: \"{}\"", (instantiated["object"]["nestedTwo"]));

}
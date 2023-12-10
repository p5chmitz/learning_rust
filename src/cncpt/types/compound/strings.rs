
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

//The Slice type
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    //For loop iterates over the converted bytes looking for the first "space"
    for (i, &item) in bytes.iter().enumerate() {
        //enumerate() returns a tuple with index i
        if item == b' ' {
            //byte literal syntx ' ' returns position i
            return &s[..i];
        }
    }
    //if the function doesn't find an empty byte ' ' it returns the whole pie
    &s[..]
}

fn main() {
    //String literals as slices
    let s: &str = "Hello, World"; //String literal
    let _sl: &str = &s[..5]; //Reference to a slice
    println!("{_sl}, suckers");

    //The String
    let mut name: String = String::new();
    name.push_str("My name is"); //Appends name
    name.push_str(" Peter"); //Appends name again
    println!("{name}");
    name.drain(10..);
    name.push_str(" not your name");
    println!("{name}");

    //Slices
    let first_wordle: String = String::from("Peter is a weirdo");
    let w: &String = &first_wordle;
    println!(
        "The first word of the phrase \"{}\" is \"{}\"",
        first_wordle,
        first_word(w)
    );

    let s: String = String::from("Hello, world!");
    let _st: &str = &s;
    println!("{_st}");
    let _hello: &str = &s[0..5];
    let _world: &str = &s[7..12];
    println!("{_hello} || {_world}");
}

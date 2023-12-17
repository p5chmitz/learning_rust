#![allow(dead_code)]
#![allow(unused_variables)]

//=======================================
//Slices

/**Declares and binds a literal as a slice.
The bytes are stored in the compiled binary.*/
pub fn string_slice_1() {
    let s: &str = "Im a slice in binary!";
    println!("{}", s);
}

/**Accesses a String vector to print slices*/
pub fn string_slice_2() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5]; //References the first 5 indexes
    let _hello = &s[1..5]; //Result is same as above
    let world = &s[7..12]; //References another 5 indexes
    println!("{hello} || {world}");

    let _hello_world = &s[..]; //References the whole index range
    println!("{_hello_world}");
}

/**Private part of the function that the book offers 
 * to illustrate the ownership-less nature of slices. Used with string_clice_4().*/
fn string_slice_3(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("Phrase as bytes: {:?}", &bytes);
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
/**Primary function of the book's parser that illustrates the 
 * onwership-less nature of slices. Works with string_slice_3().*/
pub fn string_slice_4() {
    let phrase: String = String::from("Peter is a weirdo");
    let word: &String = &phrase;
    println!(
        "The first word of the phrase \"{}\" is \"{}\"",
        phrase,
        string_slice_3(word)
    );
}

/**Re-write of the book's str parser with the help of GPT*/
pub fn string_slice_5() {
    let phrase = String::from("Schmitz is my name");
    let bytes = phrase.as_bytes();
    println!("Phrase as bytes: {:?}", &bytes);
    let mut word = String::new();
    //For loop iterates over the converted bytes looking for the first "space"
    //enumerate() returns a tuple with index i
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            break;
        }
        word.push(item as char);
    }
    println!(
        "The first word of the phrase \"{}\" is \"{}\"",
        phrase, word
    );
}

//=======================================
//String (collections)


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

    let s: String = String::from("Hello, world!");
    let _st: &str = &s;
    println!("{_st}");
    let _hello: &str = &s[0..5];
    let _world: &str = &s[7..12];
    println!("{_hello} || {_world}");
}

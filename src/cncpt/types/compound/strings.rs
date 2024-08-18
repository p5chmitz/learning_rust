#![allow(dead_code)]
#![allow(unused_variables)]

//==============
// String Slices

/** Declares and binds a literal as a slice;
The bytes are stored in the compiled binary */
pub fn string_slice_1() {
    let s: &str = "Im a slice in binary!";
    println!("{}", s);
}

/** Accesses a String vector to print slices */
pub fn string_slice_2() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5]; //References the first 5 indexes
    let _hello = &s[1..5]; //Result is same as above
    let world = &s[7..12]; //References another 5 indexes
    println!("{hello} || {world}");

    let _hello_world = &s[..]; //References the whole index range
    println!("{_hello_world}");
}

/**Primary function of the book's parser that illustrates the
 * onwership-less nature of slices. Works with string_slice_3().*/
pub fn string_slice_3() {
    let phrase: String = String::from("Peter is a weirdo");
    let word: &String = &phrase;
    println!(
        "The first word of the phrase \"{}\" is \"{}\"",
        phrase,
        string_slice_4(word)
    );
}
/**Private (associated) function that the book offers
 * to illustrate the ownership-less nature of slices. Used with string_slice_4().*/
fn string_slice_4(s: &str) -> &str {
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

/**Create Strings with literals or variables using the from() method or the to_string() methods*/
pub fn string_wrapper_1() {
    //Using the from() method
    let s = String::from("Hello");
    let mut sp = String::from(&s);
    sp.push_str(" World");
    println!("The \"from()\" method:\n  {}\n  {}", s, sp);

    //Using the to_string() method
    let data1 = "Im a literal";
    data1.to_string();
    //let data1 = "Im a litera".to_string(); // Does the same as above in one line
    let data2 = "Im also a literal".to_string();
    println!("The \"to_string()\" method:\n  {}\n  {}", data1, data2);
}
#[test]
// Shows that String can be dereferenced into a slice
fn string_wrapper_test_1() {
    let x: String = "Hello".to_string();
    let y: &str = &x;
    assert_eq!(x, y);
}

/**Uses the various mechanisms to append and concatenate strings*/
pub fn string_wrapper_2() {
    //Using the + operator uses the `add()` method
    //which requires String base with &str modifications;
    //The + operator moves values
    let s: String = String::from("Hello");
    let s_1 = s.clone();
    let s2: &str = &String::from(" world"); //The + uses add() which adds &str
    let concat = s + s2; //Moves s, s2 is already a reference
    let concat2 = s_1 + s2; //Required cloned s due to move, s2 is a reference
    println!("s + s2: {}\ns clone with reused s2: {}", concat, concat2);

    //Also uses the + operator, but with two String bases, one referenced;
    //Even though the `add()` method takes only `&str` values, we can supply
    //a &String and the compiler will coerce it into an &str
    let s3 = String::from("Hello");
    let s3_1 = s3.clone();
    let s4 = String::from(" animals");
    let s5 = " wildlings"; //Adds a literal
    let concat3 = s3 + &s4; //Compiler can coerce String to &str
    let concat4 = s3_1 + &s5; //Required clone due to move
    println!("s3 + s4: {}\ns3 clone + s5 literal: {}", concat3, concat4);

    //The push_str() method is used to push &str to mutable String base
    //I cant seem to create a new variable using just the push_str() method
    let mut s6 = String::from("Hello");
    let s7: &str = &String::from(" nerds");
    s6.push_str(s7);
    println!("s6 (modified): {}\ns7: {}", s6, s7);

    //The push() method appends a single character to a mutable string
    let mut s8 = String::from("lo");
    s8.push('l');
    println!("s8: {}", s8);
}

/**Accesses string slices*/
pub fn string_wrapper_3() {
    //Access the first 5 bytes of the string
    //This only works because of the base set of Unicode characters
    //If the text used 2-byte characters, the program might panic
    let s = String::from("Peter Schmitz");
    let given = &s[..5];
    let family = &s[6..];
    println!(
        "Full name: {}\nGiven name: {}\nFamily name: {}",
        &s, given, family
    );

    // let hello = "Здравствуйте";
    // let first_three = &hello[..4];
    // println!("First three characters: {}", first_three);

    let mut char_vec = Vec::new();
    let hello = "Здравствуйте";
    //let hello = "Peter";
    let i: usize = hello.chars().count();
    for c in hello.chars() {
        char_vec.push(c)
    }
    println!(
        "The first three chars of \"{}\" are: {:?}",
        &hello,
        &char_vec[..3]
    );
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

    let s: String = String::from("Hello, world!");
    let _st: &str = &s;
    println!("{_st}");
    let _hello: &str = &s[0..5];
    let _world: &str = &s[7..12];
    println!("{_hello} || {_world}");
}

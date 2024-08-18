#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};

//=======
// Tuples

// Declares a tuple with homogenous scalar types
pub fn tuple_1() {
    let whatev = (1, 2, 3, 4);
    let e = whatev.3;
    println!("When the tuple hits just right, {e}");

    let oneary = (1,); // Tuple with one element
                       //let oneary_again = (1); // Not a tuple, unnecessary use of ()
    let binary = (1, 2); // Tuple with two elements
}
// Declares a tuple with mixed scalar types
pub fn tuple_2() {
    let idk: (i32, f64, u8) = (32, 6.4, 8); //Creates a tuple
    let (a, b, c) = idk; //access all elements
    let d: &f64 = &idk.1; //access specific elements by index
    println!("The whole tuple: {a}, {b}, {c}");
    println!("Accessing a tuple index: {d}");
}
// Declares a tuple with mixed scalar and compound types
pub fn tuple_3() {
    let user: (bool, String, String, u64) = (
        true,
        String::from("Pschmitz"),
        String::from("peter@email.com"),
        23,
    );
    let user_name = user.1;
    let email = user.2;
    println!("Username: {user_name}");
    println!("Email: {email}");
}
// Declares a tuple with mixed scalar and compound types
pub fn tuple_4() {
    let x: (String, u8) = (String::from("Peter"), 40);
    let name: String = String::from(x.0);
    let age: u8 = x.1;
    println!("{name} is {age} years old.");
}

//=======
// Arrays

pub fn array_1() {
    //Be explicit
    let array: [f64; 3] = [32.0, 6.4, 8.0];
    let a: &f64 = &array[0]; //access specific elements by index
    println!("Accessing a tuple index: {a}");
}
pub fn array_2() {
    //Be implicit
    let array_two = [23; 5];
    let b: &i32 = &array_two[3];
    println!("Lets print a hastily initialized array index: {b}");
}
pub fn array_3() {
    let a = String::from("Peter");
    let b = String::from("Schmitz");
    let array_three: [&str; 2] = [&a, &b];
    println!(
        "My first name is {}, and my last name is {}",
        array_three[0], array_three[1]
    )
}
/** Illustrates two different ways to declare an array*/
pub fn array_4() {
    let a = [0i32, 1, 2]; // The first entry is a value and its type
    let b: [i32; 3] = [0, 1, 2];
}
/** Illustrates iterating over arrays in two different ways */
pub fn array_5() {
    // Uses idiomatic iterator object to write consecutive values to each index in the array
    let mut a_x: [i32; 10] = [0; 10];
    let mut c = 0;
    for i in a_x.iter_mut() {
        *i = c;
        c += 1;
    }

    // Uses direct indexing to write consecutive values to each index in the array
    // May be more efficient (but less idiomatic) than using iterators
    let mut a_y: [i32; 10] = [0; 10];
    for i in 0..a_y.len() {
        a_y[i] = i as i32;
    }

    println!("{:?}\n{:?}", a_x, a_y);
    println!("{}", a_y[5]);
}

/** Mirrors Java examples */
pub fn array_6() {
    #[derive(Default)]
    struct GameEntry {
        name: String,
        score: Option<usize>,
    }
    impl Clone for GameEntry {
        fn clone(&self) -> GameEntry {
            GameEntry {
                name: self.name.clone(),
                score: self.score,
            }
        }
    }
    impl GameEntry {
        // Constructs a new GameEntry object
        pub fn build(name: String, score: usize) -> GameEntry {
            GameEntry {
                name,
                score: Some(score),
            }
        }

        // Formats GameEntry objects for output
        pub fn format(&self) -> (String, String) {
            let name = self.name.clone();
            let score = self.score.map_or("".to_string(), |s| s.to_string());
            (name, score)
        }
        // For users more familiar with imperative paradigms
        //pub fn format(&self) -> (String, String) {
        //    let name = self.name.clone();
        //    let score = match &self.score {
        //        Some(s) => s.to_string(),
        //        None => return (*name, "".to_string()),
        //    };
        //    (*name, score)
        //}

        // Takes a podium array and a new entry, evaluates the entry
        // and inserts the element accordingly
        //
        // Neat but uses raw pointer manipulation in an unsafe block
        // Requires: use std::ptr
        //pub fn add(mut podium: Box<[GameEntry]>, new_entry: GameEntry) -> Box<[GameEntry]> {
        //    let mut insert_index = None;
        //    for i in 0..podium.len() {
        //        if podium[i].score.is_none() || podium[i].score < new_entry.score {
        //            insert_index = Some(i);
        //            break;
        //        }
        //    }

        //    // Requires raw pointer operations within an unsafe block
        //    if let Some(index) = insert_index {
        //        for j in (index..podium.len() - 1).rev() {
        //            unsafe {
        //                let ptr1 = &mut podium[j] as *mut GameEntry;
        //                let ptr2 = &mut podium[j + 1] as *mut GameEntry;
        //                ptr::swap(ptr1, ptr2);
        //            }
        //        }
        //        podium[index] = new_entry;
        //    }
        //    podium
        //}
        pub fn add(mut podium: [GameEntry; 3], new_entry: GameEntry) -> [GameEntry; 3] {
            // Evaluates the existing array and finds appropriate insertion index
            let mut insert_index = None;
            for i in 0..podium.len() {
                if podium[i].score.is_none() || podium[i].score < new_entry.score {
                    insert_index = Some(i);
                    break;
                }
            }
            // Shift elements to the right of the insertion index to make room
            // for the new entry; Requires Clone implementation on GameEntry struct
            if let Some(index) = insert_index {
                for j in (index..podium.len() - 1).rev() {
                    podium[j + 1] = podium[j].clone();
                }
                podium[index] = new_entry;
            }
            podium
        }
        // Takes a podium array and an index, removes the entry at the index
        // and shifts all remaining elements up by one index
        pub fn remove(mut podium: [GameEntry; 3], cheater: usize) -> [GameEntry; 3] {
            for i in cheater..podium.len() - 1 {
                podium[i] = podium[i + 1].clone();
            }
            podium[podium.len() - 1] = Default::default();
            podium
        }
    }

    // Requires the #[derive(Default)] trait for the GameEntry struct
    let mut podium: [GameEntry; 3] = Default::default();

    // Sample data to build GameEntry objects
    let names_vec: Vec<String> = vec![
        "Peter".to_string(),
        "Dingus".to_string(),
        "Brain".to_string(),
        "Bobson".to_string(),
    ];
    let scores_vec: Vec<usize> = vec![1223, 34, 616, 42069];

    // Creates a vector of GameEntry objects from sample data
    let mut entries: Vec<GameEntry> = Default::default();
    for i in 0..names_vec.len() {
        let entry: GameEntry = GameEntry::build(names_vec[i].clone(), scores_vec[i]);
        entries.push(entry);
    }

    // Adds each entry in the sample data vector to the podium and prints results
    for i in 0..entries.len() {
        podium = GameEntry::add(*Box::new(podium.clone()), entries[i].to_owned());
        println!("Add {}", &entries[i].name);
        for (c, entry) in podium.iter().enumerate() {
            let entry = entry.format();
            println!("{:>2}: {:<8} {:>6}", c + 1, entry.0, entry.1)
        }
        println!("");
    }

    // Removes an entry because Bobson isn't real
    let r = vec![0, 1, 0];
    for i in 0..r.len() {
        podium = GameEntry::remove(podium, r[i]);
        println!("Remove score at index: {}", r[i]);
        for (c, entry) in podium.iter().enumerate() {
            let entry = entry.format();
            println!("{:>2}: {:<8} {:>6}", c + 1, entry.0, entry.1)
        }
        println!("");
    }
}

/** Simple cryptography function */
pub fn array_7() {
    // Creates encoder, decoder arrays, and applies rotation for the cipher
    let mut encoder: [char; 26] = ['A'; 26];
    let mut decoder: [char; 26] = ['A'; 26];
    let rotation = 3;
    for i in 0..26 {
        encoder[i] = char::from((65 + (i + rotation) % 26) as u8);
        decoder[i] = char::from((65 + (26 + i - rotation) % 26) as u8);
    }
    //println!("Encoder: {:?}\nDecoder: {:?}", encoder, decoder);

    // Code handles up to a 64 byte plaintext message, 
    // filters out lowercase and non-ASCII characters
    let plaintext = "DONT SEARCH FOR ALL THE ANSWERS ALL AT ONCE".to_string();
    let plaintext = plaintext
        .chars()
        .filter(|&c| c.is_ascii_uppercase())
        .collect::<String>();
    let len = plaintext.len();

    // Encodes message
    let encoded_message = transformer(&plaintext, Box::new(encoder));
    println!("Encoded message: {}", encoded_message);

    // Decodes message
    let decoded_message = transformer(&encoded_message, Box::new(decoder));
    println!("Decoded message: {}", decoded_message);
}

// Handles up to a 64 byte plaintext message
fn transformer(plaintext: &String, schema: Box<[char]>) -> String {
    let mut msg_array: [u8; 64] = [0; 64];
    for (i, byte) in plaintext.trim().bytes().enumerate() {
        msg_array[i] = byte;
    }

    for k in 0..plaintext.len() {
        let j: usize = (msg_array[k] - b'A') as usize;
        msg_array[k] = schema[j] as u8;
    }

    // Convert only the valid characters in msg_array to a string
    String::from_utf8_lossy(&msg_array[..plaintext.len()]).to_string()
}

//========
// Vectors

/** Creates a Vector with 2 indexes and prints both,
 * resulting in a print statement that lists my birthday*/
pub fn vec_test_1() {
    //Requires type annotation because no type is provided for inference
    let mut v: Vec<String> = Vec::new();
    let bday: String = String::from("12/23/1983");
    let intro: String = String::from("My birthday is: ");
    v.push(intro);
    v.push(bday);
    println!("{}{}", v.get(0).unwrap(), v.get(1).unwrap());
    //println!("My birthday is: {}", v.get(n).unwrap());
}

/** Creates a vector of i32 input size.
 * The first line of output counts up from the 0 index.
 * The second line of output counts back down.*/
pub fn vec_test_2(i: i32) {
    //let mut v: Vec<i32> = Vec::with_capacity(i as usize);
    let mut index = i;
    let mut v: Vec<i32> = Vec::new();
    while index >= 1 {
        v.push(index.try_into().unwrap());
        index -= 1;
    }
    for x in v.iter().rev() {
        print!("{x} ");
        match x {
            //Requires a match guard conditional to
            //match against the external variable i;
            //Requires a de-referenced y variable
            //to compare like types (i32);
            y if *y == i => {
                println!(": That's all she wrote!");
            }
            //Only counts up to 12, but will count down all of the indexes
            12 => {
                println!(": That's all you get.");
                break;
            }
            _ => {}
        }
    }
    for x in v {
        print!("{x} ");
        match x {
            1 => {
                println!(": Blastoff!");
            }
            _ => {}
        }
    }
}
/** This function illustrates the two ways to get
 * out-of-bounds index values from a vector.
 * Warning, this method panics the program.*/
pub fn vec_test_3() {
    let v = vec![1, 2, 3]; //Vector declaration & instantiation
    let oob1 = v.get(3); //Graceful OOB handling
    match oob1 {
        Some(oob1) => println!("{oob1}"),
        None => println!("Out-of-bounds!"),
    }
    let ok = v.get(5); //Same thing works without a match but needs a formatter
    println!("{:?}", v.get(5));
    let oob2: &i32 = &v[3]; //Direct index reference panics the program
}
/** This function takes an i32, creates a vector
 * of that size, and returns the value at the halfway point.*/
pub fn vec_test_4(n: usize) {
    let mut i = n;
    let mut v: Vec<usize> = Vec::new();
    while i >= 1 {
        v.push(i);
        i -= 1;
    }
    let half = v.get(&n / 2);
    match half {
        Some(half) => println!("{}", half),
        None => println!("I guess there wasn't anything available!"),
    }
}
/** Creates a vector of usize and accesses
 * it with reference and match methods*/
pub fn vec_test_5(n: usize) {
    let mut i = 0;
    let mut v: Vec<i32> = Vec::new();
    while i <= n {
        v.push(i as i32);
        i += 1;
    }
    i = 0;
    println!("The vector has {} indexes", v.len());
    while i <= n {
        if i > n {
            break;
        };
        println!("Ref. loop: {}", &v[i]);
        i += 1;
    }
    i = 0;
    while i <= n {
        let val = v.get(i);
        match val {
            Some(val) => println!("Match loop: {}", val),
            None => {}
        }
        i += 1;
    }
}
/** The proper way to iterate over a vector using a for loop */
pub fn vec_test_6(i: usize) {
    let mut v = vec![i];
    let mut n = 1;
    while n <= i {
        v.push(n * 112);
        n += 1;
    }
    for x in v {
        println!("{}", x);
    }
    //while n <= i {
    //    v.push(n * 15);
    //    n += 1;
    //}
}
/** Illustrates how to properly index vectors */
pub fn vec_test_7() {
    // Vector v has 5 elements with indexes 0 - 4
    let v: Vec<i32> = vec![10, 20, 30, 40, 50];
    let magic_num: i32 = 50;

    // Standard for loop with STANDARD range
    // Note that using inclusive range ..= results in overflow
    for i in 0..v.len() {
        print!("{}, ", v[i]);
        if v[i] == magic_num {
            println!("End");
        }
    }

    // De-sugared for loop over iterator expression as a range
    let mut iter: std::ops::Range<usize> = (0..v.len()).into_iter();
    while let Some(i) = iter.next() {
        print!("{}, ", v[i]);
        if v[i] == magic_num {
            println!("End");
        }
    }

    // Requires value de-referencing because iter() yields
    // index references as &i32
    for i in v.iter() {
        print!("{}, ", i);
        if *i == magic_num {
            println!("End");
        }
    }

    // Alternatively into_iter() takes ownership and 
    // allows direct access to indexed values
    for i in v.into_iter() {
        print!("{}, ", i);
        if i == magic_num {
            println!("End");
        }
    }
}

//==========
// Hash Maps

/** Creates a hash map of String keys and integer values;
 * Demonstrates basic insert and check-before-write methods;
 * Briefly explores copy vs move for hash map ownership */
pub fn hash_maps_1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Democrat"), 37);
    scores.insert(String::from("Republican"), 48);
    scores.insert(String::from("Independent"), 12);
    scores.entry(String::from("Socialist")).or_insert(3);

    let party = String::from("Democrat");
    //Uses copied() because i32 implements the Copy trait
    let score = scores.get(&party).copied().unwrap_or(0);
    println!("The D's got: {}% of the vote.", score);

    //Prints hash map in arbitrary order
    for (key, value) in &scores {
        println!("The {} party has {}% of the vote.", key, value);
    }
}
/** Creates a hash map with String key and value pairs;
 * Explores hash map ownership */
pub fn hash_maps_2() {
    let mut hash = HashMap::new();
    let k1 = String::from("Democrats");
    let v1 = String::from("dirty");
    hash.insert(k1.clone(), v1);

    // Cant use copied() to get value as the previous example
    // illustrates because String doesn't implement the Copy trait;
    // Sets None value as "IDK" for return
    let default = String::from("IDK");
    let quality = hash.get(&k1).unwrap_or(&default);
    println!("The {} are described as {}", k1, quality);
}

/** Book example 8-25 that creates a hash map to count the number
of times a word appears in the supplied text; The split_whitespace()
method returns an iterator over sub-slices used to update the counter */
pub fn hash_maps_3() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}

//===========
// Book tests

/** Takes a vector of integers and calculates the mean, median, and mode;
Listing occurs end of chapter 8 on collections */
pub fn book_test_1() {
    //let mut v = vec![1, 12, 34, 23, 21, 7, 75, 12, 89, 23, 12];
    let mut v = vec![12, 98, 34, 23, 21, 4, 7, 75, 12, 89, 37, 23, 66, 12, 90, 47];
    //let mut v = vec![];
    //let mut v = vec![1, 2, 3];

    println!("\nThe vector contains {} elements:\n\t{:?}", v.len(), v);
    println!("\nAnalytics\n=========");

    if v.len() != 0 {
        //Calculates the mean
        let mut sum = 0;
        for i in &v {
            sum += i;
        }
        let mean = sum / v.len();

        //Calculates the median
        v.sort();
        let median = v[v.len() / 2];

        //Calculates the mode
        //Takes vector and creates a hash map of unique items and their number of occurences
        let mut map = HashMap::new();
        for i in &v {
            let count = map.entry(i).or_insert(0);
            *count += 1
        }
        //Pushes the hash map back into a vector of tuple values
        let mut rank: Vec<_> = map.into_iter().collect();
        //Sorts the new vector by descending value portion of the key:value tuples
        rank.sort_by(|a, b| b.1.cmp(&a.1));
        let mode: (&usize, i32) = rank[0];

        //Analytics print statements
        println!("\tThe mean is: {}", mean);
        println!("\tThe median is: {}", median);
        println!("\tThe mode and frequency of occurences is: {:?}", mode);

        //Sorts the vector by descending key portion of the key:value tuple
        rank.sort_by(|a, b| b.0.cmp(&a.0));
        //Prints range by accessing the key portion of the key:value tuple
        //as vector[index].tuple_position
        println!("\tThe range is: {}-{}\n", rank[rank.len() - 1].0, rank[0].0)
    } else {
        println!("Cant really do much with a zero-length array.");
    }
}

/** Pig Latin generator */
//let s = String::from("Hello, Im a pig latin translator.");
//let s = String::from("Im a pig latin translator");
//ello-Hay Im-ay ig-pay atin-lay ranslator-tay
//elloHay, Imyay ayay igpay atinlay anslatortray.
pub fn book_test_2() {
    print!("\nEnter phrase: ");
    io::stdout().flush().unwrap();
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failure. You're a failure.");

    let mut v = Vec::new();
    let mut output = String::new();
    for word in s.split_whitespace() {
        v.push(word);
        let first = &word[..1];
        let rest = &word[1..];
        let mut new_word: String = String::from(rest) + &first.to_string();
        let vow_pat = Regex::new(r"[aeiouAEIOU]").unwrap();
        if vow_pat.is_match(first) {
            new_word.push_str(&String::from("hay"));
        } else {
            new_word.push_str(&String::from("ay"));
        }
        //let punc_pat = Regex::new(r"[.,:!]").unwrap();
        //let punc = String::new();
        //punc.push_str();
        //rest.len();
        //if punc_pat.is_match(rest) {
        //    new_word.push_str(punc);
        //}

        output.push_str(&new_word);
        output.push_str(" ");
    }
    println!("Translation: {}\n", output);
}

/***/
pub fn book_test_3() {
    print!("Enter name, e.g. \"Walter Smith\": ");
    io::stdout().flush().unwrap();
    let mut employee: String = String::new();
    io::stdin()
        .read_line(&mut employee)
        .expect("Failure. You're a failure.");
    print!("Enter department, e.g. \"Engineering\": ");
    io::stdout().flush().unwrap();
    let mut department: String = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failure. You're a failure.");

    //let map = HashMap::new();
}

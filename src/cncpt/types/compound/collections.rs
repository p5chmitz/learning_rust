#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

//============================================
//Tuples

//Declares a tuple with homogenous scalar types
pub fn tuple_1() {
    let whatev = (1, 2, 3, 4);
    let _e = whatev.3;
    println!("When the tuple hits just right, {_e}");
}
//Declares a tuple with mixed scalar types
pub fn tuple_2() {
    let idk: (i32, f64, u8) = (32, 6.4, 8); //Creates a tuple
    let (_a, _b, _c) = idk; //access all elements
    let _d: &f64 = &idk.1; //access specific elements by index
    println!("The whole tuple: {_a}, {_b}, {_c}");
    println!("Accessing a tuple index: {_d}");
}
//Declares a tuple with mixed scalar and compound types
pub fn tuple_3() {
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
}
//Declares a tuple with mixed scalar and compound types
pub fn tuple_4() {
    let x: (String, u8) = (String::from("Peter"), 40);
    let _name: String = String::from(x.0);
    let _age: u8 = x.1;
    println!("{_name} is {_age} years old.");
}

//============================================
//Arrays

pub fn array_1() {
    //Be explicit
    let array: [f64; 3] = [32.0, 6.4, 8.0];
    let _a: &f64 = &array[0]; //access specific elements by index
    println!("Accessing a tuple index: {_a}");
}
pub fn array_2() {
    //Be implicit
    let array_two = [23; 5];
    let _b: &i32 = &array_two[3];
    println!("Lets print a hastily initialized array index: {_b}");
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

//============================================
//Vectors

/**Creates a Vector with 2 indexes and prints both,
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

/**Creates a vector of i32 input size.
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
/**This function illustrates the two ways to get
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
/**This function takes an i32, creates a vector
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
/**Creates a vector of usize and accesses
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
/**Illustrates the proper way to iterate over a vector using a for loop*/
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

//============================================
//Hash Maps

/**Creates a hash map of String keys and integer values;
 * Demonstrates basic insert and check-before-write methods;
 * Briefly explores copy vs move for hash map ownership*/
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
/**Creates a hash map with String key and value pairs;
 * Explores hash map ownership*/
pub fn hash_maps_2() {
    let mut hash = HashMap::new();
    let k1 = String::from("Democrats");
    let v1 = String::from("dirty");
    hash.insert(k1.clone(), v1);

    //Cant use copied() to get value as the previous example
    //illustrates because String doesn't implement the Copy trait;
    //Sets None value as "IDK" for return
    let default = String::from("IDK");
    let quality = hash.get(&k1).unwrap_or(&default);
    println!("The {} are described as {}", k1, quality);
}

/**Book example 8-25 that creates a hash map to count the number
 * of times a word appears in the supplied text; The split_whitespace()
 * method returns an iterator over sub-slices used to update the counter*/
pub fn hash_maps_3() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}

//============================================
//Book tests

/**Takes a vector of integers and calculates the mean, median, and mode*/
pub fn book_test_1() {
    let mut v = vec![1, 12, 34, 23, 21, 7, 75, 12, 89, 23, 12];
    println!("The vector is: {:?}", &v);

    //Calculates the median
    v.sort();
    let median = v.len() / 2;
    println!("The median is: {}", v[median]);

    //Calculates the mean
    let mut total = 0;
    let mut i = 0;
    while i < v.len() {
        total = total + &v[i];
        i += 1;
    }
    total = total / v.len() as i32;
    println!("The mean is: {}", total);

    //Calculates the mode
    let mut map = HashMap::new();
    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1
    }
    let mut rank: Vec<_> = map.into_iter().collect();
    rank.sort_by(|a, b| b.1.cmp(&a.1));
    println!("The mode and frequency of occurences is: {:?}", rank[0]);
}

/***/
pub fn book_test_2() {
    
}

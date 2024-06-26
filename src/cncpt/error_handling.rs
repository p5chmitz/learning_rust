#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
    panic,
};

// These functions panic!!!!!!

/**Illustrates simple error handling with a method that returns a Result<T, E> type;
 * This function is basically desinged to panic and doesn't really DO anything*/
pub fn error_handling_1() {
    //Creates an object of type Result<T, E> for the file's title
    let greeting_result = File::open("./files/hello.txt");

    //Creates a handler to process the Result object
    //and extract either a file or an error
    let greeting_result_handler = match greeting_result {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

/**Illustrates matching on different kinds of errors from the same Result<T, E> return;
 * This function doesn't actually DO anything but create a file if one is not found*/
pub fn error_handling_2() {
    //Sets location of test file
    let title = String::from("./files/hello.txt");
    //Creates an object of type Result<T, E> for the file's title
    let greeting_result = File::open(&title);

    //Creates a handler to pass the file object to a variable to work with
    let greeting_result_handler = match greeting_result {
        Ok(file) => file,
        //Accesses the Error::kind() method
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&title) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            //Catch-all as named identifier; The match arms must all
            //return the same type; panic! can return the File type, but
            //println doesn't return anything, so this example shows a hack around that
            //other_error => panic!("Error: {:?}", other_error),
            other_error => {
                println!("{:?}", other_error);
                File::open(" ").expect(" ")
            }
        },
    };
}

/**Introduces two methods for the Result<T, E> enum implementation
 * with unwrap() and expect(); Both of these methods use the match
 * mechanism under the hood and panic when they encounter an Err;
 * Because of this, their use is generally discouraged by the Rust
 * language documentation; This function does the same thing as
 * error_handling_1() but combines the result with the result handler*/
// This shit panics
pub fn error_handling_3() {
    //let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt").expect("hello.txt is required to proceed");
}

/**Uses error propagation to let the calling code handle errors;
 * Requires the use std::io::self statement;*/
pub fn error_handling_4() -> Result<String, io::Error> {
    let process_result = File::open("hello.txt");
    let mut file_handler = match process_result {
        Ok(file) => file,
        //If an error is encountered the function returns the error
        //instead of panicking
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file_handler.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        //Does the same thing as the previous match expression,
        //but we dont need an explicit return keyword Because
        //this is the last expression in the function
        Err(e) => Err(e),
    }
}

/**This function does the same as error_handling_4() but uses
 * the ? operator to dispatch with the boilerplate of the
 * match expressions*/
pub fn error_handling_5() -> Result<String, io::Error> {
    let mut process_result = File::open("./files/hello_world.txt")?;
    let mut contents = String::new();
    process_result.read_to_string(&mut contents)?;
    Ok(contents)
}

/**Error handling with closures; This code does the same thing as error_handling_2() but uses
 * closures instead of match expressions*/
pub fn error_handling_7() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

/** rust-lang documentation example of File::open usage*/
pub fn error_handling_8() -> std::io::Result<()> {
    let mut file = File::open("./src/hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //assert_eq!(contents, "Hello!");
    Ok(())
}

/***/
pub fn error_handling_9() {
    //Sets location of test file
    let title = String::from("./files/hello.txt");
    //Creates an object of type Result<T, E> for the file's title
    let greeting_handle = File::open(&title);

    let greeting_file = match greeting_handle {
        Ok(file) => file,
        Err(e) => panic!("TESTING: {:?}", e.kind()),
    };
}

/**The most elegant way to read a file's contents in a self-contained
 * way without panicking*/
pub fn idk() {
    let contents = match fs::read_to_string("./fils/hello_world.txt") {
        Ok(contents) => println!("idk3: {}", contents),
        Err(e) => println!("Error: {}...{}", e, e.kind()),
    };
}

/**Using the ? operator with the Option type*/
pub fn error_handling_10() -> Option<char> {
    let s = String::from("Im a phrase");
    s.lines().next()?.chars().last()
}

// These functions DONT panic!!!!!!

/**This function does the same thing as error_handling_3 and
 * error_handling_4 but does so in a ridiculously short way;
 * I cant believe it took so long to get here.
 * This function must be implemented with a Result match type.*/
pub fn error_handling_6() -> Result<String, io::Error> {
    fs::read_to_string("./src/cncpt/hello.txt")
}

pub fn error_handling_11() {
    //Option 1
    let file_contents = match std::fs::read_to_string("./files/hello_world.txt") {
        Ok(file) => file,
        Err(e) => format!("Error: {}", e),
    };
    println!(
        "error_handling_11() (with trailing print):\n\t{}\n",
        file_contents
    );

    //Option 2
    let file_contents = match std::fs::read_to_string("./files/hello_world.txt") {
        Ok(file) => file,
        Err(e) => {
            let error_message = format!("Error: {}", e);
            println!(
                "error_handling_11() (with integrated print & early return): \n\t{}\n",
                error_message
            );
            return;
        }
    };
    println!("Unreachable with an error");
}

pub fn error_handling_12() {
    let mut file_contents = String::new();
    if let Ok(file) = std::fs::read_to_string("./files/hello_world.txt") {
        //file_contents.push_str(&file) //Requires allocation and copying, which may be slower
        file_contents = file;
    } else {
        println!("error_handling_12() (using if let Ok syntax): \n\tError: [cannot access error message]\n");
        return;
    };
    println!("Unreachable with an error:\n\t{}", file_contents);
}

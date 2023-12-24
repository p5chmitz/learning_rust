#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
        fs::File, 
        io::{
            Read, 
            ErrorKind
        }, 
        panic};

/**Illustrates simple error handling with a method that returns a Result<T, E> type;
 * This function is baseically desinged to panic and doesn't really DO anything*/
pub fn error_handling_1() {
    //Creates an object of type Result<T, E> for the file's title
    let greeting_result = File::open("./files/hello.txt");
    
    //Creates a handler to process the Result object 
    //and extract either a file or an error 
    let greeting_result_handler = match greeting_result {
        Ok(file) => file,
        Err(error) => panic!("Problem creating the file: {:?}", error),
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
            //Catch-all as named identifier
            //other_error => panic!("Error: {:?}", other_error),
            other_error => {
                println!("{:?}", other_error);
                File::open(" ").expect(" ") 
            }
        },
    };
}

/**rust-lang documentation example of File::open usage*/
pub fn error_handling_3() -> std::io::Result<()> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello!");
    Ok(())
}

/***/
pub fn error_handling_4() {
    //Sets location of test file
    let title = String::from("./files/hello.txt");
    //Creates an object of type Result<T, E> for the file's title
    let greeting_handle = File::open(&title);

    let greeting_file = match greeting_handle {
        Ok(file) => file,
        Err(e) => panic!("TESTING: {:?}", e.kind()),
    };
}

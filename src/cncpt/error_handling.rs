use std::{
    fs::{File, OpenOptions},
    io::Read,
};

pub fn error_handling_1() {
    //panic!("Failure message");
    //NOTE: When testing in terminal ensure that run commands
    //are executed from the /src/ dir, otherwise a file not found
    //error may occur.
    let mut file_contents = String::new();
    let open_file_result = OpenOptions::new()
        .read(true)
        .open("./files/hello_world.txt");
    //Less verbose version (alias) of the previous line(s)
    //let open_file_result = File::open("./files/hello_world.txt");
    let file_handler = match open_file_result {
        Ok(mut file) => match file.read_to_string(&mut file_contents) {
            Ok(_) => {}
            Err(err) => eprintln!("Error: {}", err),
        },
        Err(error) => panic!("Error: {:?}", error),
    };
    println!("{:?}", file_handler);
    println!("{}", file_contents);
}

pub fn error_handling_2() {
    //Creates String to buffer file contents
    let mut file_contets = String::new();
    //Accesses the file (with options)
    let file = OpenOptions::new()
        .read(true)
        .open("./files/hello_world.txt");
    //Accesses file handle, copies text to String buffer
    match file {
        Ok(mut file) => {
            println!("Opening File...");
            match file.read_to_string(&mut file_contets) {
                Ok(_) => println!("Contents copied to String..."),
                Err(err) => println!("There was a problem: {}", err),
            }
        }
        Err(err) => println!("Its a failure, for sure. {}", err),
    };
    //Prints contents of String buffer
    println!("Printing String contents...\n\t{}", file_contets);
}

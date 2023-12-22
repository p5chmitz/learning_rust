//NOTE: When testing in terminal ensure that run commands
//are executed from the /src/ dir, otherwise a file not found
//error may occur.

use std::{fs::OpenOptions, io::Read};

/**Accesses a file with proper error handling via classic OpenOptions method*/
pub fn io_1() {
    //Creates String to read contents into
    let mut file_contents = String::new();
    //Creates file handle
    //Less verbose version (alias) of OpenOptions
    //let open_file_result = File::open("./files/hello_world.txt");
    let file_handle = OpenOptions::new()
        .read(true)
        .open("./files/hello_world.txt");
    //Accesses file handle, copies contents to String
    match file_handle {
        Ok(mut file) => {
            println!("Accessing file & reading to String...");
            //Reads file contents to String
            match file.read_to_string(&mut file_contents) {
                Ok(_) => {}
                Err(error) => eprintln!("Read error: {:#?}", error),
            }
        }
        Err(error) => eprintln!("File error: {:#?}", error),
    };
    println!("File contents via proper method:\n\t{}", file_contents);
}

/**Accesses a file the EASY way; This pattern substitutes 
 * the expect text for the desired file contents instead of 
 * failing in a meaningful way*/
pub fn io_2() {
    let contents = std::fs::read_to_string("./files/hello_world.txt")
        .expect("[placeholder text]");
    println!("The easy way...\n\t{}", contents)
}

/**Accesses a file with some super hacky bullshit;
 * This function includes two variations that use the
 * std::fs::read_to_string method;
 * If the file path is incorrect all options will trigger panic*/
pub fn _io_3() {
    println!("Heres some hacky bullshit!\n==========================");
    //Option 1
    let file_contents = match std::fs::read_to_string("./files/hello_world.txt") {
        Ok(file) => file,
        Err(_) => String::from("Your shits all fucked up"),
    };
    println!("Option 1:\n\t{}", file_contents);

    //Option 2
    let mut file_contents = String::new();
    if let Ok(file) = std::fs::read_to_string("./files/hello_world.txt") {
        file_contents = file;
    } else {
        println!("Im a stupid error")
    };
    println!("Option 2:\n\t{}", file_contents);

}


use std::{fs::File, io::Read};

pub fn error_handling_1() {
    //panic!("Failure message");
    //NOTE: When testing in terminal ensure that run commands
    //are executed from the /src/ dir, otherwise a file not found
    //error may occur.
    let mut file_contents = String::new();
    let open_file_result = File::open("./files/hello_world.txt");
    let file_handler = match open_file_result {
        Ok(mut file) => {
            match file.read_to_string(&mut file_contents) {
                Ok(_) => {},
                Err(err) => eprintln!("Error: {}", err),
            }
        },
        Err(error) => panic!("Error: {:?}", error),
    };
    println!("{:?}", file_handler);
    println!("{}", file_contents);
}

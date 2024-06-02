use std::thread;
use std::time::Duration;

// Concurrency with closures
pub fn concurrency_1() {

    // Data
    let mut v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = vec!["a", "b", "c", "d", "e"];
    let v3 = vec!["one", "two", "three", "four", "five"];

    // Thread 1
    let t_1 = thread::spawn(move || {
        for e in v1.iter_mut() {
            *e *= *e;
            println!("Thread 1: {e} ");
            thread::sleep(Duration::from_secs(1))
        }
    });

    // Thread 2
    let t_2 = thread::spawn(move || {
        for e in v3.iter() {
            println!("Thread 2: {e} ");
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Main thread logic
    for e in v2.iter() {
        println!("Main: {e} ");
        thread::sleep(Duration::from_secs(1))
    }

    // Ensures that all threads finish before exiting the function
    t_1.join().expect("Uh oh");
    t_2.join().expect("Uh oh");

    println!("All done!")
}

// Concurrency with message passing
//
// Creates data in main thread, moves it to new thread,
// new thread transmits each vector element to the message channel,
// the main thread contains a channel receiver that polls the
// channel for data until it finds a stop indication,
// then the function formats the received chunks as a message
//
// Uses recv and try_recv
use std::sync::mpsc;
use std::io::{self, Write};
pub fn concurrency_2() {
    // Multiple producer, single consumer channel initialization
    let (tx, rx) = mpsc::channel();
    // OG message(s)
    let val = String::from("am OG");
    let val_vec = vec![
        "henlo".to_string(),
        "am".to_string(),
        "big".to_string(),
        "stinky".to_string(),
        "or".to_string(),
        "loquatious".to_string(),
        "vec".to_string(),
        "messg".to_string(),
        // stop acts as keyword to stop message receipt loop
        "stop".to_string(),
    ];

    // Creates a new thread and moves the data where 
    // it is sent as timed elements of the vector
    thread::spawn(move || {
        for e in val_vec.iter(){
            tx.send(e.clone()).unwrap();
            thread::sleep(Duration::from_millis(3000))
        }

        thread::sleep(Duration::from_secs(5));
        //tx.send(val).unwrap();
    });

    // recv() blocks the main thread until it gets a value
    //let received = rx.recv().expect("No dice");

    // My shitty attempt at polling
    // try_recv() doesn't block and returns an error if no value is present
    let mut message: Vec<String> = vec![];
    loop {
        match rx.try_recv() {
            Ok(m) => {
                // loop ends when the handle receives the stop message
                if m == "stop".to_string() {
                    break
                }
                //println!("\n{m}");
                message.push(m)
            },
            Err(e) => {
                // Broke-dick animation job while the loop polls the rx handle
                let states = ["Waiting   ", "Waiting.  ", "Waiting.. ", "Waiting..."];
                for state in states.iter() {
                    print!("{}\r", state);
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_millis(200));
                }
            },
        };
    }

    // Formats and prints the final message from constituent vector elements
    let mut formatted = String::new();
    for e in message {
        formatted.push_str(" ");
        formatted.push_str(&e)
    }
    println!("Final message: {}", formatted.trim());
}


// Does literally the same thing as the previous method but treats
// the receiver handle like an iterator and consumes it in a for loop;
// When the channel is closed the iteration ends
pub fn concurrency_3() {

    // Multiple producer, single consumer channel initialization
    let (tx, rx) = mpsc::channel();

    // OG message(s)
    let val = String::from("am OG");
    let val_vec = vec![
        "Using".to_string(),
        "recv".to_string(),
        "as".to_string(),
        "an".to_string(),
        "iterator".to_string(),
    ];

    // Creates a new thread and moves the data where 
    // it is sent as timed elements of the vector
    thread::spawn(move || {
        for e in val_vec.iter(){
            tx.send(e.clone()).unwrap();
            thread::sleep(Duration::from_millis(500))
        }
        // tx goes out of scope, is dropped, and closes the channel
    });

    // Treats rx like an iterator
    // Consumes the receiver tho ¯\_(ツ)_/¯
    for e in rx {
        println!("Recv sez: {e}")
    }
}

// Does the same as the 3rd iteration but clones the transmitter
pub fn concurrency_4() {

    // Multiple producer, single consumer channel initialization
    let (tx, rx) = mpsc::channel();

    // OG message(s)
    let val = String::from("am OG");
    let val_vec = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
    ];

    // Creates a new thread and moves the data where 
    // it is sent as timed elements of the vector
    let tx_clone = tx.clone();
    let val_clone = val_vec.clone();
    thread::spawn(move || {
        for e in val_clone.iter(){
            tx_clone.send(e.clone()).unwrap();
            thread::sleep(Duration::from_millis(250))
        }
        // tx goes out of scope, is dropped, and closes the channel
    });

    thread::spawn(move || {
        for e in val_vec.clone().iter(){
            tx.send(e.clone()).unwrap();
            thread::sleep(Duration::from_millis(500))
        }
        // tx goes out of scope, is dropped, and closes the channel
    });

    // Treats rx like an iterator
    // Consumes the receiver tho ¯\_(ツ)_/¯
    for e in rx {
        println!("rx sez: {e}")
    }

}



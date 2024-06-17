// Slices
/////////
pub fn slices_1() {
    let a = "Hello, Rust".to_string(); // Creates a string
    let slice = &a[7..]; // Creates a reference to a substring slice of type &str
    assert_eq!(slice, "Rust"); // Compares the slice to a static string "Rust"
}
// Slices are views into contiguous sequences, which means they can be iterated
pub fn slices_2() {
    let s = "Hello, Rust".to_string(); // Creates a string
    let mut shout = String::new(); // Creates a mutable string to push new values
    for c in s[7..].chars() { // chars() creates an iterator from a string slice (&str)
        shout.push(c.to_ascii_uppercase())
    }
    assert_eq!(shout, "RUST".to_string()); // Proves the result
    println!("{}", s); // chars() takes a reference so s is still valid
}

// References
/////////////

// Illustrates an immutable reference
pub fn references_1() {
    let x = 12;
    let y = &x; // Creates an immutable reference to x
    assert_eq!(*y, 12); // Uses the dereference operator to compare referenced value
}

// Illustrates mutable references using scope and a closure
pub fn references_2() {
    // Simple mutable reference with inner scope
    let mut x = 23; // Creates a mutable variable
    {
        let y = &mut x; // Creates a mutable reference to the original variable
        *y += 60; // Mutates the mutable reference, effectively mutating the original variable
    }
    assert_eq!(x, 83); // Proves that x has changed

    // Does the same thing as above but with a closure
    let mut a = 23;
    let mutate = |x: &mut i32| *x + 60; // Creates a closure that takes a mutable reference
    a = mutate(&mut a); // Passes a mutable reference to the closure, changing a
    assert_eq!(a, 83); // Proves that a is now changed
}

/** Illustrates immutable and mutable references */
pub fn references_3() {
    // Heap-allocated value
    let val = String::from("Peter"); 
    // Creates an immutable reference to the value
    let val_ref = &val; 
    // Prints the referenced object
    println!("Reference: {}", val_ref); 
    assert_eq!(*val_ref, "Peter".to_string()); // Need to deref &String to compare to String
    assert_eq!(val_ref, "Peter"); // Rust can automatically coerce &String to &str

    // Creates a mutable heap-allocated value
    let mut stringy = String::from("Peter");
    // Creates a mutable reference to the value
    let stringy_ref: &mut String = &mut stringy;
    // Modifies the value through a mutable reference
    stringy_ref.push_str(" is pretty OK");
    // Prints the referenced object
    println!("Reference: {}", stringy_ref);
}

// Illustrates the dereference operator on a borrowed value
pub fn references_4() {
    let v = vec!['a', 'b', 'c'];
    // enumerate() is an iterator adapter that takes an iterator and returns an 
    // iterator that yields Option<(usize, <I as Iterator>::Item)>
    // The loop body prints each index and its dereferenced value as 
    for (i, val) in v.iter().enumerate() {
        println!("Element {} holds value {}", i, *val)
    }
}

// Pointers
///////////


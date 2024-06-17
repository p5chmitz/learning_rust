pub fn unsafe_1() {
    let mut num = 5;
    let r1 = &num as *const i32; // Raw pointer!
    let r2 = &mut num as *mut i32; // Raw pointer!

    unsafe {
        println!("{:#?}", r1);
        println!("{:?}", *r2);
    }
}

/** Illustrates immutable and mutable references */
pub fn unsafe_2() {
    // Heap-allocated value
    let val = String::from("Peter"); 
    // Creates an immutable reference to the value
    let val_ref = &val; 
    // Prints the referenced object
    println!("Reference: {}", val_ref); 

    // Creates a mutable heap-allocated value
    let mut stringy = String::from("Peter");
    // Creates a mutable reference to the value
    let stringy_ref: &mut String = &mut stringy;
    // Modifies the value through a mutable reference
    stringy_ref.push_str(" is pretty OK");
    // Prints the referenced object
    println!("Reference: {}", stringy_ref);
}

pub fn unsafe_3() {
    // Heap-allocated value
    let mut val = String::from("Peter"); 
    
    // Creates a raw pointer to the value 
    // Points to the underlying String object named in the reference &val
    let val_ptr: *const String = &val; 
    // Pretty prints the raw pointer as a memory address
    println!("Raw pointer address: {:#?}", val_ptr); 
    unsafe {
        // Dereferences the raw pointer to print the value at the address
        println!("Dereferenced raw pointer value: {}", *val_ptr) 
    }

    val.push_str(" is pretty OK");
    unsafe {
        // Dereferences the raw pointer to print the value at the address
        println!("Third dereferenced raw pointer value: {}", *val_ptr);
    }
}

#[allow(unused_unsafe)]
pub fn unsafe_4() {
    let val = "Peter".to_string();
    let p: *const String = &val;
    unsafe {
        // Illegal: cant assign pointer to data through an immutable raw pointer
        //*p = "".to_string();  
    }
}

// Creates a raw pointer to an arbitrary address
// Causes a segmentation fault 😊
pub fn unsafe_5() {
    let address = 0x00007ff7ba76070cusize;
    let r = address as *const i32;
    unsafe {
        println!("{:?}", *r)
    }
}

static PETER: &str = "Peter"; // Immutable global static variable (constant)
static mut NAME: &str = "Michael"; // Mutable global static variable

// Unions are mostly for interop with C
union Peter {
    x: i32,
    y: u8,
    z: [u8; 8],
}


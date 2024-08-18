pub fn unsafe_1() {
    let mut num = 5;
    let r1 = &num as *const i32; // Raw pointer!
    let r2 = &mut num as *mut i32; // Raw pointer!

    unsafe {
        println!("{:#?}", r1);
        println!("{:?}", *r2);
    }
}

/** Illustrates immutable raw pointer */
pub fn unsafe_3() {
    // Heap-allocated value
    let mut val = String::from("Peter");

    // Creates an immutable raw pointer to the value
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
    unsafe { println!("{:?}", *r) }
}

static PETER: &str = "Peter"; // Immutable global static variable (constant)
static mut NAME: &str = "Michael"; // Mutable global static variable

// Unions are mostly for interop with C
union Peter {
    x: i32,
    y: u8,
    z: [u8; 8],
}

pub fn hr(arr: &[i32]) -> (f32, f32, f32) {
    let v = arr.to_vec();
    let mut rat: (f32, f32, f32) = (0.0, 0.0, 0.0); // As pos, neg, zer
    for e in v.iter() {
        if *e > 0 {
            rat.0 += 1.0
        } else if *e < 0 {
            rat.1 += 1.0
        } else {
            rat.2 += 1.0
        }
    }
    let r1 = rat.0 / v.len() as f32;
    let r2 = rat.1 / v.len() as f32;
    let r3 = rat.2 / v.len() as f32;
    println!("{}\n{}\n{}", r1, r2, r3);
    return (r1, r2, r3);
}

#[test]
pub fn hr_test() {
    let a = [1, 1, 0, -1, -1];
    assert_eq!(hr(&a), (0.4, 0.4, 0.2))
}

pub fn unsafe_6() {
    let loans: Vec<i32> = vec![
        633147, 413361, 625448, 547422, 412630, 899222, 138597, 900119, 257699, 889214,
    ];
    let i32sum = loans.iter().sum::<i32>();
    let f32sum = i32sum as f32 / 100.0;
    let months = f32sum as i32 / 500;
    let years = months / 12;
    println!("Total balance: {}", f32sum);
    println!("Total months at $500/mo: {}", months);
    println!("Total years at $500/mo: {}", years);
}

// loan calculator
pub fn loans() {
    struct Loan {
        name: String,
        school: String,
        loan_status: String,
        balance: f32,
        original_principal: f32,
        interest_rate: f32,
        loan_type: String,
        loan_id: String,
    }

    let loans: Vec<Loan> = vec![];
}

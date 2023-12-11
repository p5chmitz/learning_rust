#![allow(dead_code)]
#![allow(unused_variables)]

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

//Vectors
/**Prints my birthday*/
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
The first line of output counts up from the 0 index.
The second line of output counts back down.*/
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
            //Requires a match guard conditional to match against the external variable i
            //Requires a dereferenced y variable to compare like types (i32)
            y if *y == i => {
                println!(": Thats all she wrote!");
            }
            //Only counts up to 12, but will count down all of the indexes
            12 => {
                println!(": Thats all you get.");
                break;
            }
            _ => {}
        }
    }
    for x in &v {
        print!("{x} ");
        match x {
            1 => {
                println!(": Blastoff!");
            }
            _ => {}
        }
    }
}


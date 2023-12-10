//Its tuples!
pub fn tuple() {
    //declares a tuple of mixed types
    let idk: (i32, f64, u8) = (32, 6.4, 8); //Creates a tuple
    let (_a, _b, _c) = idk; //access all elements
    let _d: &f64 = &idk.1; //access specific elements by index
    println!("The whole tuple: {_a}, {_b}, {_c}");
    println!("Accessing a tuple index: {_d}");

    //Declares a User tuple
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

    //declares a tuple of the same type
    let whatev = (1, 2, 3, 4);
    let _e = whatev.3;
    println!("When the tuple hits just right, {_e}");
}

pub fn tuple_two() {
    let x: (String, u8) = (String::from("Peter"), 40);
    let _name: String = String::from(x.0);
    let _age: u8 = x.1;
    println!("{_name} is {_age} years old.");
}

//Its an array, Carlie Brown!
pub fn array() {
    //Be explicit
    let array: [f64; 3] = [32.0, 6.4, 8.0];
    let _a: &f64 = &array[0]; //access specific elements by index
    println!("Accessing a tuple index: {_a}");
    //Be implicit
    let array_two = [23; 5];
    let _b: &i32 = &array_two[3];
    println!("Lets print a hastily initialized array index: {_b}");
    let a = String::from("Peter");
    let b = String ::from("Schmitz");
    let array_three: [&str; 2] = [&a, &b];
    println!("My first name is {}, and my last name is {}", array_three[0], array_three[1])
}

pub fn vec_test_1() {
    //Adds type annotation because no type is provided for inference
    let mut v: Vec<String> = Vec::new();
    let bday: String = String::from("12/23/1983");
    let intro: String = String::from("My birthday is: ");
    v.push(intro);
    v.push(bday);
    println!("{}{}", v.get(0).unwrap(), v.get(1).unwrap());
    //println!("My birthday is: {}", v.get(n).unwrap());
}

pub fn vec_test_2(mut i: i32) {
    //let mut v: Vec<i32> = Vec::with_capacity(i as usize);
    let mut v: Vec<i32> = Vec::new();
    println!("i is: {i}");
    while i >= 1 {
        v.push(i.try_into().unwrap());
        i -= 1;
    }
    println!("i is: {i}");
    for x in &v {
        println!("{x}");
        match x {
            1 => {
                println!("Blastoff!");
            }
            _ => {}
        }
    }
    println!("i is: {i}");
}
// Defines custom trait Format
pub trait Format {
    fn format(&self) -> String;
}

// Defines the struct Person
pub struct Person {
    name: String,
    age: u8,
    height: f32,
}
// Implements Display
//impl std::fmt::Display for Person {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{} {} {}", self.name, self.age, self.height)
//    }
//}
// Implements custom trait Format
impl Format for Person {
    fn format(&self) -> String {
        format!(
            "{}\n\tage: {}\n\theight: {} cm",
            self.name, self.age, self.height
        )
    }
}

// Defines the struct House
pub struct House {
    address: String,
    age: u8,
    bedrooms: u8,
    bathrooms: u8,
}
// Implements Display
impl std::fmt::Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.address, self.age, self.bedrooms, self.bathrooms
        )
    }
}
// Implements custom trait Format
impl Format for House {
    fn format(&self) -> String {
        format!(
            "\n\taddress: {}\n\tage: {}\n\tbedrooms: {}\n\tbathrooms: {}",
            self.address, self.age, self.bedrooms, self.bathrooms
        )
    }
}

// Simple use of struct with Display implemented
pub fn traits_1() {
    let patient = Person {
        name: "Peter".to_string(),
        age: 40,
        height: 175.26,
    };
    //println!("Patient: {}", patient);
    println!("Patient: {}", Person::format(&patient));
}

// Illustrates trait objects with dynamic dispatch

// The trait
pub trait MyTrait {
    fn my_method(&self) -> String;
}

// The struct and the trait implementation
struct MyStruct;
impl MyTrait for MyStruct {
    fn my_method(&self) -> String {
        String::from("Hello from MyStruct")
    }
}

// A function that uses dynamic dispatch to create a trait object
use std::fmt::Display;
fn print_message(item: &dyn Display) {
    println!("{}", item);
}

fn traits_2() {
    let my_struct = MyStruct;
    //print_message(&my_struct);
}

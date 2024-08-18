// Illustrates:
// 1) Define a custom trait
// 2) Implements the custom trait on a custom struct
// 3) Implements a standard trait on a custom struct
// 4) Puts it all together in a public function
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

/////////////////////////////////////////////////////

// Illustrates trait objects with dynamic dispatch
// 1) Defines a custom trait
// 2)

// Declares a custom trait
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

//=========================
// Illustrates custom types

//===============================
// Default trait types
// The std::ops::Add trait includes a default type for
// overloading an addition operation;
// It says that if you dont specify the right-hand side of an
// addition operation, Rust assumes its the same as the left side;

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Uses the default Rhs type as Self which implements Point + Point
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn traits_4() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

#[derive(Debug, PartialEq)]
pub struct Millimeters(u32);
pub struct Meters(u32);

// Specifies Meters as an Rhs override which implements Millimeters + Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[test]
pub fn traits_test() {
    // Asserts the default setup is true
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Sets up a modified operation
    let mm = Millimeters(23);
    let m = Meters(12);
    let result: Millimeters = mm.add(m);
    println!("{:#?}", result);
    assert_eq!(result, Millimeters(12023));
}

//=============================================
// Illustrates fully qualified trait syntax(es)

trait Wonky {
    // required method
    fn into_iter(&self);
    // required associated function
    fn wonk();
}
trait Wonk {
    // required associated function
    fn wonk();
}
struct IDK {
    a: Vec<i32>,
}
impl IDK {
    fn into_iter() {
        println!("Just another print statement")
    }
}
impl Wonky for IDK {
    fn into_iter(&self) {
        println!("Not your grandmothers iterator!");
    }
    fn wonk() {
        println!("Wonky associated function");
    }
}
impl Wonk for IDK {
    fn wonk() {
        println!("Another wonky associated function");
    }
}
impl IntoIterator for IDK {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter()
    }
}

pub fn opener() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
}

/** Illustrates fully qualified method syntax */
pub fn calling_code() {
    let bare_bones = IDK { a: vec![4, 5, 6] };
    // Default method syntax
    bare_bones.into_iter();
    // Fully qualified syntax for inherent method
    // This does the same thing as the default syntax
    IDK::into_iter();

    let vec = vec![1, 2, 3];
    let object = IDK { a: vec };
    // Fully qualified syntax calls the IntoIterator trait method
    let iterator = IntoIterator::into_iter(object);
    for e in iterator {
        println!("{}", e)
    }

    let another_vec = vec![1, 2, 3];
    let another_object = IDK { a: another_vec };
    // Fully qualified syntax calls the Wonky trait method
    Wonky::into_iter(&another_object);

    // Fully qualified syntax specifies the wonk() function
    // for the Wonk trait
    <IDK as Wonk>::wonk();

    // Fully qualified syntax specifies the wonk() function
    // for the Wonky trait
    <IDK as Wonky>::wonk();
}

//========================
// Illustrates supertraits

use std::{fmt, io};

// OutlinePrint can only be implemented on types that
// also implement Display
// Display is a supertrait of OutlinePrint
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct SuperPoint {
    x: i32,
    y: i32,
}
// Implements a custom trait
impl OutlinePrint for SuperPoint {}

// Implements the custom trait's supertrait
impl fmt::Display for SuperPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn calling_code_2() {
    let p = SuperPoint { x: 12, y: 23 };
    p.outline_print();
}



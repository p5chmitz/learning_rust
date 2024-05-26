#![allow(unused_assignments)]
//use std::thread;
//use std::{collections::HashMap, io::Write};

// These three functions illustrate the control flow of closures
// Change the String s to change the outcome of the process
pub fn closures_driver() {
    let s: String = String::from("");
    let e = error_message();
    println!("{}", closures_1(&s).unwrap_or_else(|| &e));
}
pub fn closures_1(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
pub fn error_message() -> String {
    let s = String::from("Aint nothin here!");
    s
}

// Creates a list of custom enums to demonstrate closures somehow
#[derive(Debug)]
enum Category {
    Electronics,
    Music,
    Books,
    Clothing,
}
#[derive(Debug)]
struct List {
    stuff: Vec<Category>,
}
impl List {
    fn builder() -> List {
        List {
            stuff: vec![
                Category::Electronics,
                Category::Music,
                Category::Books,
                Category::Books,
                Category::Clothing,
            ],
        }
    }
    // The only proper closure here
    fn option(&self, pref: Option<Category>) -> Category {
        pref.unwrap_or_else(|| self.most())
    }
    fn most(&self) -> Category {
        let mut e = 0;
        let mut m = 0;
        let mut b = 0;
        let mut c = 0;
        for item in &self.stuff {
            match item {
                Category::Electronics => e += 1,
                Category::Music => m += 1,
                Category::Books => b += 1,
                Category::Clothing => c += 1,
            }
        }
        //let mut h = HashMap::new();
        //h.insert("Electronics", e);
        //h.insert("Music", m);
        //h.insert("Books", b);
        //h.insert("Clothing", c);
        //println!("The inventory: {:?}", h);
        let mostest = e;
        if m > mostest {
            m = mostest;
            return Category::Music;
        }
        if b > mostest {
            b = mostest;
            return Category::Books;
        }
        if c > mostest {
            c = mostest;
            return Category::Clothing;
        } else {
            return Category::Electronics;
        }
    }
}
#[test]
fn category_test() {
    let store = List::builder();
    let most = store.most();
    println!("Store inventory: {:?}", store);
    println!("The most is: {:?}", most);
    println!(
        "I have no preference so Im getting: {:?}",
        store.option(None)
    );

    // Calls a List method
    let result_3 = store.most();
    println!("Result 3: {:?}", result_3);
    // Does the same thing, but with a closures
    let result_2 = || store.most();
    println!("Result 2: {:?}", result_2());
}

// Demonstrates closure syntax
fn double_it(mut n: i32) -> i32 {
    n += n;
    n
}
#[test]
fn closure_syntax_test() {
    let x = 10;

    // Creates an anonymous function closure_1 that takes one argument val
    let closure_1 = |val| double_it(val);
    assert_eq!(closure_1(x), 20);

    // Creates an anonymous function closure_2 that takes no arguments
    let closure_2 = || x * 2;
    assert_eq!(closure_2(), 20)
}

// Demonstrates how a closure is used in the move operation
#[test]
fn moving() {
    // Forces ownership move to a new thread
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    std::thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// Demonstrates the use of a closure in the sort_by_key method
#[test]
fn sort_by_key() {
    let mut v = vec![1, 3, 5, 4, 2];
    v.sort_by_key(|&x| x);
    //v.sort();
    println!("Sorted vec: {:?}", v);
    assert_eq!(v, [1, 2, 3, 4, 5]);

    let mut c = vec!['P', 'e', 't', 'e', 'r'];
    c.sort_by_key(|&x| x);
    println!("Sorted vec: {:?}", c);
    assert_eq!(c, vec!['P', 'e', 'e', 'r', 't'])
}

// Creates a struct and sorts it with a closure
#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[test]
fn rectangles() {
    let mut rectangles = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    rectangles.sort_by_key(|r| r.width);
    println!("{:#?}", rectangles);
    let sorted = [
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
        Rectangle {
            width: 10,
            height: 1,
        },
    ];
    assert_eq!(sorted, rectangles);
}

#![allow(dead_code)]
#![allow(unused)]

// The Box<T> type
//////////////////
// Allocates data on the heap

// Instantiates a Box
pub fn smart_pointers_1() {
    let b = Box::new(5);
    print!("Box b: {}", b);
}

// Implements a cons (constructor) list
pub enum Cons {
    Cons(i32, Box<Cons>),
    Nil,
}

// Illustrates that Box'd variables can be dereferenced like any heap reference
#[test]
fn smart_pointers_2() {
    let a: i8 = 5;
    let b = Box::new(a);
    assert_eq!(5, *b);
    assert_eq!(5, *b.deref()); // deref() only works because Box implements Deref
}

// Illustrates dereferencing and coercion of Box variables
#[test]
fn smart_pointers_3() {
    // Compares a &str with a &str
    let a = "Hello";
    assert_eq!("Hello", a);

    // Automatically coerces a String to a &str during comparison
    let b = "Hello".to_string();
    assert_eq!("Hello", b);

    // Dereferences the String and coerces to &str during comparison
    let c = Box::new("Hello".to_string());
    assert_eq!("Hello", *c);
}

use std::collections::HashMap;
// Defines a custom smart pointer like Box<T>
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// Implements the external Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// The Rc<T> type
/////////////////
// Provides multiple ownership (immutable access) and strong reference

use std::rc::Rc;

use rand::distributions::Distribution;

// Increase strong count with Rc::clone
pub fn ref_counter() {
    let a = Rc::new("Hello");
    println!(
        "There are {} live references to \"{}\".",
        Rc::strong_count(&a),
        &a
    );
    {
        // Increase the strong count with Rc::clone
        let b = Rc::clone(&a);
        println!(
            "There are {} live references to \"{}\".",
            Rc::strong_count(&a),
            &a
        );
    }
    println!(
        "There are {} live references to \"{}\".",
        Rc::strong_count(&a),
        &a
    );
}

// Interior mutability
/////////////////////////////////////////////

// Creates a LimitTracker using a defined Messenger trait
// to illustrate a use case for interior mutability
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    // Creates new LimitTracker object
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// The RefCell<T> type
//////////////////////
// For all your internal mutability and weak reference needs

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Illustrates a memory leak with a simple Cons list
pub mod mem_leak {
    use crate::cncpt::types::smart_pointers::mem_leak::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    // Like main, but just for the module
    pub fn driver() {
        // Sets ref count a to (5, Nil)
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        // Sets b to (10, a)
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        // Modify a to point to b instead of Nil,
        // creates a cycle that overflows the stack when accessing the tail
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        //println!("a next item = {:?}", a.tail());
    }
}

// Illustrates how to avoid cyclic references with a simple tree and Weak references
pub mod weak_ref {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    pub fn driver() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
}

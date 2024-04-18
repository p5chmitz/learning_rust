#![allow(dead_code)]
#![allow(unused)]

// Instantiates a Box
pub fn smart_pointers_1() {
    let b = Box::new(5);
    print!("Box b: {}", b);
}

// Implements a cons (construct function) list
pub enum Cons {
    Cons(i32, Box<Cons>),
    Nil,
}

#[test]
pub fn smart_pointers_2() {
    let x: String = String::from("Hello");
    let y: &str = &x;

    // compares String with &str
    assert_eq!(String::from("Hello"), y);
    assert_eq!(x, y);

    let a: i8 = 5;
    let b = Box::new(a);
    assert_eq!(5, *b);
    assert_eq!(5, *b.deref());

    let oneary = (1,);
    let oneary_again = (1);
    let binary = (1, 2);
}

fn caller() {
    let n = String::from("Peter");
    hello(&(*n)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}")
}

// Defines a custom smart pointer
use std::ops::Deref;

struct MyBox<T>(T);

// Implements the MyBox type
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implements external traits for the type with required method(s)
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::rc::Rc;
pub fn ref_counter() {
    let a = Rc::new("Hello");
    println!(
        "There are {} live references to \"{}\".",
        Rc::strong_count(&a),
        &a
    );
    {
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

/* Does a bunch of shit **/
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

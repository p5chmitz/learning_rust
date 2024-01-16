#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

//Adding generic lifetime annotation guarantees that the returned reference will be valid as long
//as both arguments are valid; Describes the relationship between the argument(s) and the return
//value
pub fn lifetimes_1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
pub fn lifetimes_2<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Borrowed<'a> {
    owned: String,
    borrowed: &'a str,
}
impl<'a> Borrowed<'a> {
    fn _level(&self) -> i32 {
        23
    }
}
pub fn caller_2() {
    let s = String::from("world");
    let example = Borrowed {
        owned: String::from("Hello"),
        borrowed: &s,
    };
    println!("Together they make: {} {}", example.owned, example.borrowed);
    let s: &str = "hello";
}

pub fn caller_1() {
    let s1 = String::from("Hello");
    let s2 = "world";
    let result = lifetimes_1(&s1, s2); //function requires references
    println!("The longer string is {}", result)
}

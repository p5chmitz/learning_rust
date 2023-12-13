#![allow(dead_code)]
#![allow(unused_variables)]

//use std::io;
//use std::str;

/**The function takes a reference type (&String) and returns its length*/
fn calc_len(s: &String) -> usize {
    let i: usize = s.len();
    return i;
}
/**Illustrates the difference between stack and heap-allocated memory using one type that
 * implements the Copy trait, and one that does not. This is similar to the idea between a
 * "shallow" copy (the Copy trait) and a "deep" copy in other languages.*/
pub fn ownership_1() {
    let a: i32 = 32;
    let b: i32 = a;
    println!("{a}");
    let ass: String = String::from("I like to move it, move it");
    let mut bass = ass.clone();
    bass.push_str("!"); //requires clone because of push_str() move
    println!("{ass}");
    println!("{bass}");
}
pub fn ownership_2() {
    let mut v = vec![1, 2, 3];
    //let a = &v[2]; //Immutable borrow
    v.push(4); //Mutable borrow causes immutable borrow to get deallocated
    let a = &v[2]; //Immutable borrow
    println!("{}", a); //Requires either shadowed allocation or mutable borrow, otherwise immutable borrow is out-of-scope

    let mut s = String::from("Peter");
    //let a = &s; //Immutable borrow
    s.push_str("!"); //Mutable borrow
    let a = &s; //Immutable borrow
    println!("{a}"); //Illegal?
}
pub fn ownership_3() {
    let s: String = String::from("Hello");
    let cloned_var: String = s.clone();
    println!("Modified greeting: {}", ownership_3a(cloned_var));
    println!("Original greeting: {}", s);
}
fn ownership_3a(i: String) -> String {
    let mut a: String = i;
    a.push_str(" world");
    return a;
}
pub fn ownership_4() {
    let s = String::from("Peter");
    let s1 = ownership_3a(s);
    println!("{}", s1);
}
fn ownership_4a(s: String) -> String {
    s
}


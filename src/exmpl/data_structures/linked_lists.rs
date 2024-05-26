#![allow(dead_code)]
#![allow(unused_variables)]
//#![allow(unused_imports)]

// Singly linked list
#[derive(Debug)]
pub struct List<T> {
    head: Option<Node<T>>,
    size: i32,
}
impl<T: std::fmt::Debug> List<Node<T>> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
    pub fn size(&self) -> i32 {
        self.size
    }
    fn push_to_head(&mut self, data: T) {
        let new = Node::new(data);
        //TODO: Make this work
        self.size += 1;
    }
}
// Node struct
#[derive(Clone, Debug)]
pub struct Node<T> {
    data: T,
    reference: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            reference: None,
        }
    }
    fn add(self, data: T) -> Node<T> {
        Node {
            data,
            reference: self.reference,
        }
    }
}
pub fn singly_linked_list() {
    // Creates a new linked list
    let mut list: List<Node<String>> = List::new();

    // Creates a head node and pushes it to the list
    let s1 = "Iowa".to_string();
    //let head: Node<String> = Node::new(s1);
    list.push_to_head(s1);

    let s2 = "Colorado".to_string();
    //let mut node: Node<String> = Node::new(s2);
    list.push_to_head(s2);

    let s3 = "Illinois".to_string();
    //node = Node::new(s3);
    list.push_to_head(s3);

    let s4 = "Oregon".to_string();
    //node = Node::new(s4);
    list.push_to_head(s4);

    println!(
        "Linked list is {} elements long and looks like this:\n\t{:#?}",
        list.size(),
        list
    );
}

struct Rectangle {
    width: u32,
    height: u32,
}
// Struct implementation
impl Rectangle {
    // Constructor as an associated function
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn double(x: u32) -> u32 {
        x * 2
    }
    // "Methods" take struct instance "self" as first parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn rectangle() {
    //Instantiates a struct
    let rekt = Rectangle::new(12, 23);
    //Uses method syntax to call area method implementation
    println!("The area of rectangle is {}.", rekt.area());
    let rekt = own_n_pass(rekt);
    println!("The area of rectangle is {}.", &rekt.area());

    let idk = Rectangle::new(Rectangle::double(2), Rectangle::double(4));
    let out = idk.area();
    println!("Double rekt: {}", out);
}

fn own_n_pass(x: Rectangle) -> Rectangle {
    Rectangle {
        width: x.width * 2,
        height: 6,
    }
}

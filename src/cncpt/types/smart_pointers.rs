
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


    use std::thread;

use std::{collections::HashMap, io::Write};



pub fn closures_1(s: &str) -> Option<&str> {
   if s.is_empty() {
        None
   } else {
        Some(s)
   } 
} 

pub fn printinator() -> String {
    let s = String::from("Aint nothin here!");
    s
}

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
            stuff: vec![Category::Electronics, Category::Music, Category::Books, Category::Books, Category::Clothing]
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
        }
        else {
            return Category::Electronics;
        }
    }
}
fn testing(mut n: i32) -> i32 {
    n += n;
    n
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
fn first() {
    let store = List::builder();
    let most = store.most();
    println!("Store inventory: {:?}", store);
    println!("The most is: {:?}", most);
    println!("I have no preference so Im getting: {:?}", store.option(None));

    // Calls a List method
    let result_3 = store.most();
    println!("Result 3: {:?}", result_3);
    // Does the same thing, but with a closures
    let result_2 = || store.most();
    println!("Result 2: {:?}", result_2());

    // The closure captures its environment so we dont need 
    // to pass x to the closure when calling it
    let x = 10;
    let closure_1 = |x| testing(x);
    let closure_2 = || x * 2;
    println!("Closure: {}", closure_2());

    // Forces ownership move to a new thread
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut v = vec![1, 3, 5, 4, 2];
    //v.sort_by_key(|&x| x);
    v.sort();
    println!("Sorted vec: {:?}", v);

    //let mut t = vec![(1, 11), (3, 31), (5, 51), (4, 41) , (2, 21)];
    let mut t = vec!['P', 'e', 't', 'e', 'r'];
    t.sort_by_key(|&x| x);
    println!("Sorted vec: {:?}", t);

    let mut rectangles = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    rectangles.sort_by_key(|r| r.width);
    println!("{:#?}", rectangles);

}

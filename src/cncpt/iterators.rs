
pub struct Iter<'a> {
    data: Option<Vec<&'a str>>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.data {
            Some(i) => i.pop(),
            None => None,
        }
    }
}
pub fn iter_test() {
    let i = Iter {
        data: Some(vec!["a", "b", "c", "d"])
    };
    for e in i {
        println!("{e}")
    }
}

// Creates an iterator from an array with iter()
pub fn iterators_1() {
    println!("Iterators 1");
    let a = ["a", "b", "c", "d"];

    // Creates an iterator with an explicit call to iter()
    for val in a.iter() { 
        println!("{}", val) 
    }
    assert_eq!(a, ["a", "b", "c", "d"]);

    // Creates an iterator with implicit call to iter()
    for val in a { 
        println!("{}", val) 
    }
    assert_eq!(a, ["a", "b", "c", "d"]);

    let x = vec![1i32, 2, 3];
    let mut y: Vec<i32> = vec![];
    for val in x.iter() {
        y.push(val * val)
    }
    assert_eq!(y, [1, 4, 9]);
    //assert_eq!(x, y);
    println!()
}

// Iterators over vectors
pub fn iterators_2() {
    println!("Iterators 2");

    let a: Vec<String> = vec!["Hello".to_string(), "world".to_string()];
    let iterator = a.iter();

    // Creates an iterator from a vector with iter()
    for val in a.iter() { 
        println!("{}", val) 
    }
    // Because iter() takes &T the original values are still accessible
    assert_eq!(a, vec!["Hello".to_string(), "world".to_string()]);

    // Same as above with implicit call to into_iter()
    for val in a { 
        println!("{}", val) 
    }
    // Illegal due to iterator creation
    //assert_eq!(a, ["a", "b", "c", "d"]);
}

// Playing with a vector's mutable references
pub fn iterators_3() {
    println!("Iterators 3");

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for e in v.iter_mut() {
        println!("{e}");
        *e = 420;
    }
    println!("{:?}", &v);

    println!()
}

pub fn iterators_4() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a = [-0i32, -1, -2, -3, -4, -5, 6, -7, -8, 9, -10];
    for e in a.iter().filter(|e| e.is_positive()) {
        println!("{e}");
    }
}

// For loop de-sugaring
pub fn iterators_5() {
    let values = vec![1, 2, 3, 4, 5];
    for e in values.into_iter() {
        println!("{e}")
    }

    let values = vec![1, 2, 3, 4, 5];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                let next;
                match iter.next() {
                    Some(val) => next = val,
                    None => break,
                };
                let x = next;
                let () = { println!("{x}"); };
            },
        };
        result
    }

    let v = vec![1, 2, 3, 4];
    let i = v.iter();
    let sum: i32 = i.sum();
    assert_eq!(sum, 10);

}

pub fn iterator_example() {
    let a = ["a", "b", "c", "d"];
    for val in vec!["one", "two", "three"].iter() { 
        println!("{}", val) 
    }

    let n = 10;
    for val in 1..=n { 
        println!("{val}") 
    }

    let mut counter = 0;
    let mut boolean = false;
    while boolean == false {
        println!("Still false...");
        counter += 1;
        if counter == 10 {
            println!("Finally!");
            boolean = true
        }
    }

    let mut c = 10;
    loop {
        println!("{c}");
        if c <= 1 {
            println!("Blast off!");
            break
        }
        c -= 1;
    }

    for val in (1..=10).rev() { 
        println!("{val}") 
    }
    println!("Blast off!");

    let mut i: i32 = 10;
    while i >= 1 {
        println!("{i}");
        i -= 1;
    };
    println!("Blast off!");

    // Explicitly creates an iterator object
    // val is of type &str
    let mut y: Vec<&str> = Vec::new();
    for val in a.iter() { 
        y.push(val);
        println!("{}", val) 
    }
    assert_eq!(a, ["a", "b", "c", "d"]);

    // The &str val can be accessed as a reference or derefenced for its underlying value
    for val in a.iter() { 
        println!("{}", *val) 
    } 
    assert_eq!(a, ["a", "b", "c", "d"]);

    // Implicitly creates an iterator object with a reference to a 
    // val is of type &&str and can be dereferenced to &str
    for val in [1, 2, 3].iter() {
        println!("{}", *val) 
    }    
    assert_eq!(a, ["a", "b", "c", "d"]);

    let a: [&str; 4] = ["a", "b", "c", "d"];
    for val in a.iter() { 
        println!("{val}") // Prints the values of the array
    }

    // Implicitly creates an iterator object with a reference to a 
    // val is of type &&str and can be accessed directly
    for val in &a { 
        println!("{}", val) 
    }    
    assert_eq!(a, ["a", "b", "c", "d"]);

    //assert_eq!(a, ["a", "b", "d", "d"]);

    let x = vec!["a", "b", "c"];
    let mut y: Vec<&str> = vec![];
    for val in x {
        y.push(val)
    }
    assert_eq!(y, ["a", "b", "c"]);
    //assert_eq!(x, ["a", "b", "c"]);

}


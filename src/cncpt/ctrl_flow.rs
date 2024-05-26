#![allow(dead_code)]
#![allow(unused_variables)]
//Re-exported to use in main.rs as cncpt::loops::_my_age_static() for example

pub mod ifs {
    /**Takes a float64 and prints the range*/
    pub fn if_statements(n: f64) {
        if n < 0.33 {
            println!("lower third")
        } else if (0.33 < n) && (n < 0.66) {
            println!("middle third")
        } else if 0.66 < n {
            println!("upper third");
        }
    }
    pub fn if_statements_1(n: f64) -> String {
        let mut result = String::new();
        if n <= 0.0 || n >= 1.0 {
            result.push_str("Out of range; 0.0..1.0");
            return result;
        }
        if n < 0.0 || n > 1.0 {
            panic!("Must be between 0-1")
        }
        if n < 0.33 {
            result.push_str("lower third");
            return result;
        } else if (0.33 < n) && (n < 0.66) {
            result.push_str("middle third");
            return result;
        } else {
            result.push_str("upper third");
            return result;
        }
    }

    #[test]
    fn if_statements_test() {
        let answer = String::from(if_statements_1(0.34));
        assert_eq!("middle third", answer);
    }

    /**Takes a float64 and returns a String range*/
    fn so_many_times(n: f64) -> String {
        let mut s: String = String::new();
        if n < 0.33 {
            s = String::from("lower third");
        } else if (0.33 < n) && (n < 0.66) {
            s = String::from("mids");
        } else if 0.66 < n {
            s = String::from("upper third");
        };
        return s;
    }

    /** Uses an if statement in a declaration.

    `let _i: &str = if n < 5 { "true" } else { "false" };`
     */
    pub fn again_lets_if(n: i32) {
        let _i: &str = if n < 5 { "true" } else { "false" };
        println!("{_i}");
    }
}
pub mod loops {
    /**Introduces the basic loop keyword*/
    fn _loops() {
        let mut i: i32 = 12;
        loop {
            println!("{i}");
            i += 1;
            if i == 23 {
                println!("{i} <- nice");
                i += 1;
                continue;
            }
            if i == 30 {
                println!("The max is: {i}");
                break;
            }
        }
    }

    //Uses a loop in a declaration
    fn _loop_lets() {
        let mut i: i32 = 12;
        let _x: i32 = loop {
            println!("{i}");
            i += 1;
            if i == 23 {
                println!("{i} <- nice");
                i += 1;
                continue;
            }
            if i == 30 {
                break i;
            }
        };
        println!("The max is: {_x}");
    }

    //Uses loop labels for nested/scoped identification
    fn _loop_labels() {
        let mut count: i32 = 1;
        loop {
            println!("{count}");
            let mut second_count: i32 = 101;
            'second_loop: loop {
                println!("{second_count}");
                second_count += 1;
                if second_count == 110 {
                    break 'second_loop;
                }
            }
            count += 1;
            if count == 4 {
                break;
            }
        }
    }

    //While loops, yo
    fn _while_loops() {
        let mut i: i32 = 10;
        while i >= 1 {
            println!("{i}");
            i -= 1;
        }
        println!("Blast off!");
    }

    //Playing around with while loops
    fn _my_age() {
        let mut age: i32 = 0;
        let mut year: i32 = 1983;
        println!("I was born in {}.", &year);
        year += 1;
        while year <= 2023 {
            println!(
                "For most of {} I was {}, but at the very end I turned {}.",
                year,
                age,
                (&age + 1)
            );
            age += 1;
            year += 1
        }
    }

    //Just a static age calculator
    pub fn _my_age_static() {
        let year: i32 = 2023;
        let mut _age: i32 = year - 1984;
        println!(
            "For most of {} I was {}, but at the very end I turned {}.",
            year,
            _age,
            (_age + 1)
        )
    }

    //Just a static age calculator that takes a variable
    fn _my_age_again(year: i32) {
        let mut _age: i32 = year - 1984;
        println!(
            "For most of {} I was {}, but at the very end I turned {}.",
            year,
            _age,
            (_age + 1)
        );
    }

    //For loop with an array
    fn _for_loops() {
        let a: [&str; 4] = ["a", "b", "c", "d"];
        for _index in a {
            println!("{_index}")
        }
        for _i in (1..10).rev() {
            println!("{_i}")
        }
        println!("Blorst off!");
    }
}

#[test]
// Illustrates creating an iterator object as v_iter,
// then the function consumes the object with an iterator adapter sum()
fn iterators_1() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    let v_cons: i32 = v_iter.sum();
    assert_eq!(v_cons, 6);
}

#[test]
// Illustrates iter_mut() to mutate elements of a vector in place
fn iterators_2() {
    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        *i *= *i
    }
    assert_eq!(v, [1, 4, 9])
}

#[test]
// Illustrates two ways of iterating over and mutating the values of a vector;
//
// The first approach uses the for loop to iterate over dereferenced
// values using the iter_mut() adapter; The resultant object is consumed by
// the for loop;
//
// The second approach does the same thing but uses the iter() and
// map() adapters; The map() adapter takes a closure which does the same thing
// as the for loop in the previous approach; The resultant object is
// then consumed with the consuming adapter collect(),
fn iterators_3() {
    let v = vec![1i32, 2, 3];

    let mut v1 = v.clone();
    for i in v1.iter_mut() {
        *i *= *i
    }
    assert_eq!(v1, [1, 4, 9]);

    let v2 = v.clone();
    let v_sq: Vec<i32> = v2.iter().map(|x| x * x).collect();
    assert_eq!(v_sq, [1, 4, 9])
}

#[test]
fn iterators_4() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterators_5() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect(); // Using the + oerator in the expression implicitly de-references x to match the owned literal
    let v2: Vec<i32> = v1.iter().map(|&x| x).collect(); // Implicitly dereferences x
    let v2: Vec<i32> = v1.iter().map(|x| *x).collect(); // Explicitly dereferences x
    let v2: Vec<&i32> = v1.iter().map(|x| x).collect();
    //let v2: Vec<i32> = v1.into_iter().map(|x| x).collect();
    let v2: Vec<i32> = v1.into_iter().map(|x| x + 1).collect();

    //let v_1 = v1.iter(); // Creates an iterator over &i32 values
    //let v_2 = v_1.map(|x| x);
    //let v_3: Vec<i32> = v_2.collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

pub mod match_like {
    //Matches against the None and Some enums of the Option<T> type
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), //Executes code based on a particular enum match
        }
    }
    pub fn idk_anymore() {
        let five: Option<i32> = Some(5); //Binds a variable with data in an Option<i32> type
        let none: Option<i32> = None;
        plus_one(five);
        plus_one(Some(11));
        plus_one(none);
        plus_one(None);

        let answer: String = match plus_one(five) {
            Some(a) => format!("The answer is: {a}"),
            None => format!("Sorry, I didn't find anything"),
        };
        println!("{answer}");
    }

    use std::io::{self, Error, ErrorKind::InvalidInput};
    fn divide(x: isize, y: isize) -> Result<isize, io::Error> {
        if x == 0 {
            Err(Error::new(InvalidInput, "Dividend cannot be zero"))
        } else if y == 0 {
            Err(Error::new(InvalidInput, "Divisor cannot be zero"))
        } else {
            Ok(x / y)
        }
    }

    fn divide_by(x: isize, y: isize) -> Result<isize, &'static str> {
        if x == 0 {
            Err("Dividend cannot be zero")
        } else if y == 0 {
            Err("Divisor cannot be zero")
        } else {
            Ok(x / y)
        }
    }

    pub fn calling_function() {
        let x = 8;
        let y = 3;
        let answer = match divide_by(x, y) {
            Ok(a) => a,
            Err(e) => {
                println!("Error: {e}");
                0
            }
        };
        println!("{} / {} = {}", x, y, answer.to_string());
    }
}

pub mod if_let {

    pub fn if_let_1(n: isize) {
        if let (0..=3) = n {
            println!("The value is in range")
        } else {
            println!("The value is not in range")
        }
    }

    pub fn if_let_2() {}

    enum Message {
        Contents(String),
        Timestamp(String),
    }

    pub fn if_let() {
        // Creates a message
        let s: String = String::from("A rising tide lifts all the homies");

        // Instantiates an enum m and binds the string to the Contents variant
        let m = Message::Contents(s);

        // Uses if let to access the Contents value for the m Message object
        if let Message::Contents(result) = m {
            println!("The message is: {result}")
        }
    }
}

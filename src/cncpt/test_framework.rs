#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::Neg;
#[cfg(test)]


#[test]
pub fn test_1() {
    let result: i32 = 2 + 2;
    assert_eq!(result, 4);
}

pub fn hex_digit_finder(mut n: i32) -> Vec<String> {
    let mut r: i32;
    let mut q: i32;
    let mut v: Vec<String> = Vec::new();
    while n > 0 {
        if n <= 16 {
            v.push(n.to_string());
            break
        }
        q = n / 16;
        r = n % 16;
        v.push(r.to_string());
        n = q;
    }
    let mut v_index = v.len();
    let mut v_new: Vec<String> = Vec::new();
    while v_index > 0 {
        let x = v.get(v_index - 1);
        match x {
            Some(x) => v_new.push(x.to_owned()),
            None => (), 
        }
        v_index -= 1;
    }
    return v_new;
}

///////////////////////////////////////////////
// Function tests
pub fn function_tests_1(){
     let y = 42069;
    println!("{} converted to hex is {:?}", y, hex_digit_finder(y));
}
pub fn function_tests() {

    println!("{}", crate::cncpt::ctrl_flow::ifs::if_statements_1(0.23));
    super::generics::generics_1();
    super::generics::generics_6();
    let tweet = crate::Tweet {
        username: String::from("pschmitz"),
        content: String::from("Twitter is mostly just depressed millenial jokes"),
        reply: false,
        retweet: true,
    };
    let news_article = super::generics::NewsArticle {
        headline: String::from("The oppression of indiginous communities"),
        location: String::from("North Dakota"),
        author: String::from("Peter Schmitz"),
        content: String::from(
            "This is gonna be super long bro Im not entirely sure you're ready for this yet",
        ),
    };

    crate::cncpt::generics::generics_7(&tweet);
    crate::cncpt::generics::generics_8(&tweet, &news_article);
    crate::cncpt::generics::generics_8(&tweet, &tweet);

    //calling function for trait-bound syntax function
    let s = String::from("Hello, Peter");
    crate::cncpt::generics::generics_11(&s);

    let p = super::generics::Pair { a: 12, b: 23 };
    println!(
        "External trait on generic local type with trait bound syntax
\tThe sum of {} and {} is {}.",
        &p.a,
        &p.b,
        p.add()
    );
    println!(
        "External trait on generic local type with where clause
\tThe sum of {} and {} is {}.",
        &p.a,
        &p.b,
        p.addington()
    );

    let a = 12;
    let b = 23;
    println!(
        "Generic parameters in independent function with trait bound syntax
\tThe sum of {} and {} is {}",
        &a,
        &b,
        crate::cncpt::generics::addotron_1(a, b)
    );
    println!(
        "Generic parameters in independent function with where clause 
\tThe sum of {} and {} is {}.",
        &a,
        &b,
        crate::cncpt::generics::addotron_2(a, b)
    );
    let pair = crate::cncpt::generics::Pair { a: "12", b: "23" };
    //println!("Wow, Im an idiot: {}", &pair.printme());

}

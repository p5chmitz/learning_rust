#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::Neg;
use crate::sandbox::experiments;

#[cfg(test)]


#[test]
pub fn test_1() {
    let result: i32 = 2 + 2;
    assert_eq!(result, 4);
}

///////////////////////////////////////////////
// Function tests
pub fn function_tests_1(){
    let y = 1223;
    println!("{} converted to hex place values is {:?}", y, experiments::hex_digit_finder(y));
    //println!("{} converted to a hex string is {}", y, experiments::number_to_hex(y));
    let v = vec![1223, 69, 420, 187];
    for i in v {
        println!("{} converted to a hex string is {}", i, experiments::int_to_hex(i));
    }
    let b: u8 = 169;
    println!("{} converted to a binary string is {}", b, experiments::int_to_bin(b));
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

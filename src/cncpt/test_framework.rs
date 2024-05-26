#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::sandbox::data_conversions;
use std::ops::Neg;

#[cfg(test)]
#[test]
pub fn test_1() {
    let result: i32 = 2 + 2;
    assert_eq!(result, 4);
}

///////////////////////////////////////////////
// Function tests
pub fn function_tests_1() {
    // Converts decimal to hex
    let v = vec![1223, 69, 420, 187];
    for i in v {
        println!("{} to hex: {}", i, data_conversions::int_to_hex(i));
    }

    // Converts hex to decimal
    println!("\nConverts hex to decimal");
    let hex = String::from("1F8f");
    println!(
        "{} to decimal: {:?}",
        &hex,
        data_conversions::hex_to_int(&hex)
    );
    println!(
        "{} to decimal: {:?}",
        &hex,
        data_conversions::hex_to_int_2(&hex)
    );

    // Converts decimal to binary
    println!("\nConverts decimal to binary");
    let b: i32 = 666;
    println!("{} to binary: {}", b, data_conversions::int_to_bin(b));

    // Converts binary to decimal
    println!("\nConverts binary to decimal");
    let example = String::from("0101100101");
    println!(
        "{} as u32: {}",
        &example,
        data_conversions::bin_to_int(&example)
    );
    let example = String::from("1010011010");
    println!(
        "{} as u32: {}",
        &example,
        data_conversions::bin_to_int(&example)
    );

    // Converts string to binary
    println!("\nConverts string to binary");
    let s = String::from("Hello");
    println!("{:?}", data_conversions::string_to_val(&s));
    let s = String::from("Murder");
    println!(
        "\"{}\" to binary: \n\t{}",
        &s,
        data_conversions::int_to_bin_str_2(data_conversions::string_to_val(&s))
    );

    let s = String::from("a8b3d");
    println!(
        "{} to binary {}",
        &s,
        data_conversions::int_to_bin(data_conversions::hex_to_int_2(&s) as i32)
    );
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

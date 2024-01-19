#![allow(dead_code)]
#![allow(unused_variables)]
//#[cfg(test)]

//Use statement necessary for calling code on trait examples
//use crate::cncpt::types::traits::{Summary, NewsArticle, Tweet};
use crate::util::time;

//============================================
// Intro to generics

/** Calling function to illustrate the role of generics; Creates two vectors of different base types and passes them to a function that finds the largest item in the collection; One function is concrete through-and-through, the other has generic parameters and a where clause to only accept orderable sets*/
pub fn generics_1() {
    let list_1 = vec![34, 50, 25, 12, 65];
    let processed_1 = generics_2(&list_1);
    println!(
        "The largest via abstraction (and vectors) is {}",
        processed_1
    );
    let list_2 = vec!['d', 'c', 'a', 'b', 'z'];
    let processed_2 = generics_3(&list_2);
    println!(
        "The largest via abstraction (and arrays) is {}",
        processed_2
    );
}

/** This function takes a reference to a collection and iterates over it to find and return a reference to the largest value in the collection; Concrete example to show where we could use generics if we wanted to expand functionality*/
pub fn generics_2(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        //Requires trait definition to work
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
/** This function does the same thing as generics_2() but is defined with generic parameters and a where clause that acts as a type checker to ensure that only types with the PartialOrd trait are accepted;*/
pub fn generics_3<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        //Requires trait definition to work
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

//============================================
// Defining generic structs (the Point example)

/** Defines a generic struct over type T; The struct has two fields of the same type (T); When instantiated,
 * both struct fields must be of the same concrete type*/
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
//Illustrates how to implement a concrete method; Not used
impl Point<i32> {
    fn get_x(&self) -> &i32 {
        &self.x
    }
}
//Implements a generic set of methods
impl<T> Point<T> {
    /**Returns the x field of the struct instance*/
    fn x_getter(&self) -> &T {
        &self.x
    }
    /**Takes two struct instances, swaps the y field
    in the first instance for the y value from the
    second instance and returns a new instance*/
    fn mixup<P>(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
/**Calling function to illustrate generic type Point<T> and its methods, which consist of a simple getter and a generic method mixup<P>(); Concrete values of two different scalar types used for generic struct*/
pub fn generics_4() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x_getter());
    let first = Point { x: 12, y: 23 };
    let second = Point { x: 23, y: 12 };
    println!("First: {:?}\nSecond: {:?}", first, second);
    let combination = first.mixup::<i32>(second);
    println!("Combination: {:?}", combination);
}

/**Book example of mixing struct generics with its method generics*/
//The struct has two distinct types X1 and Y1
#[derive(Debug)]
struct BookPoint<X1, Y1> {
    x: X1,
    y: Y1,
}
//The implementation block mirrors the struct
impl<X1, Y1> BookPoint<X1, Y1> {
    //The method is a generic that defines two new generics X2 and Y2
    //The method takes
    fn mixup<X2, Y2>(self, other: BookPoint<X2, Y2>) -> BookPoint<X1, Y2> {
        BookPoint {
            x: self.x,
            y: other.y,
        }
    }
    fn mixup_2(self, other: BookPoint<X1, Y1>) -> BookPoint<X1, Y1> {
        BookPoint {
            x: self.x,
            y: other.y,
        }
    }
}
pub fn generics_5() {
    let p1 = BookPoint { x: 5, y: 10.4 };
    let p2 = BookPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Combo: {:?}", p3);
    //println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    let p4 = BookPoint { x: 69, y: 42.0 };
    let p5 = BookPoint {
        x: "holler",
        y: 'P',
    };
    let p6 = p4.mixup(p5);
    println!("Combo: {:?}", p6);
}

//===================================
// Trait declaration (the Tweet and NewsArticle examples)

/**The trait is Metadata, each behavior of the trait is represented by an "incomplete" function that
 * is ultimately defined by the implementing type; Any type with the Metadata trait will have access
 * to the functions defined within the trait block*/
pub trait Metadata {
    fn default(&self) -> String {
        format!(
            "\n\tAuthor: {}\n\tTime: {}",
            &self.author(),
            time::static_time(8)
        )
    }
    fn author(&self) -> String;
    fn summarize(&self) -> String;
}

//===================================
// News article

/**Struct NewsArticle implements the Metadata trait*/
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//Implements the Metadata trait for the NewsArticle type
impl Metadata for NewsArticle {
    fn author(&self) -> String {
        String::from(&self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//===================================
// Tweet

/**Struct Tweet implements the Metadata trait*/
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//Implements the Metadata trait for the Tweet type
impl Metadata for Tweet {
    fn author(&self) -> String {
        String::from(&self.username)
    }
    fn summarize(&self) -> String {
        format!("\n\t{}: {}", self.username, self.content)
    }
}

//===================================
// Calling code

/**Creates two structs that implement summarize() and default() traits, prints them*/
pub fn generics_6() {
    //Instantiates a news article to summarize
    let news_article = NewsArticle {
        headline: String::from("The oppression of indiginous communities"),
        location: String::from("North Dakota"),
        author: String::from("Peter Schmitz"),
        content: String::from(
            "This is gonna be super long bro Im not entirely sure you're ready for this yet",
        ),
    };
    let news_default = news_article.default();
    let news_summary = news_article.summarize();
    //Instantiates a tweet to summarize
    let tweet = Tweet {
        username: String::from("pschmitz"),
        content: String::from(
            "This is a tweet so its gonna be a bit shorter than a news article. Its mostly jokes.",
        ),
        reply: false,
        retweet: true,
    };
    let tweet_default = tweet.default();
    let tweet_summary = tweet.summarize();
    println!(
        "News article default: {}\nNews article summary: {}\nTweet default: {}\nTweet summary: {}",
        news_default, news_summary, tweet_default, tweet_summary
    );
}

//========================================
// Using traits to write functions that accept many types

/**Illustrates the "impl trait" syntax as a function parameters; This parameter accepts any type that implements the specified trait; In the body we can call any methods defined on the parameter trait, in this case our previously-defined Metadata trait; Attempting to pass any types that do not implement Metadata will cause compiler errors*/
pub fn generics_7(item: &impl Metadata) {
    println!("Breaking Twitter news! {}", item.summarize());
}
/**Takes two elements of potentially different types, both of which that implement Metadata; For
 * example, we can pass a Tweet and a NewsArticle to this function*/
pub fn generics_8(a: &impl Metadata, b: &impl Metadata) {
    println!("Breaking Twitter news! {}", a.summarize());
    println!("Breaking Twitter news! {}", b.default());
}

/**Illustrates the trait bound syntax; Does the same thing as generics_7() but without the syntax sugar; This form more clearly defines the generic function parameter*/
pub fn generics_9<T: Metadata>(item: &T) {
    println!("Written with trait bound syntax: {}", item.default());
}
/**Takes two elements OF THE SAME TYPE that both implement the Metadata trait; For example, we
 * cannot pass a Tweet and NewsArticle type to this function, they both have to be one or the other*/
pub fn generics_10<T: Metadata>(a: &T, b: &T) {
    println!("The thing we thought: {}", a.default());
    println!("The thing we thought: {}", b.default());
}

//Test for where clause on generic parameter
pub fn generics_11<S>(s: &S)
where
    S: std::fmt::Display,
{
    println!("Generic types without trait-bound syntax: {}", s)
}

//========================================
// Implementing traits on local types

//Local (generic) type declaration with two generic fields
#[derive(Debug)]
pub struct Pair<T> {
    pub a: T,
    pub b: T,
}
//Local trait declaration with one generic method
// Apparently this is REALLY local, as in not accessible from elsewhere in the crate outside of
// main.rs and here.
pub trait PrintMe {
    fn printme(&self) -> String;
}
//Implementing local trait on generic type with impl trait syntax
impl<T: ToString> PrintMe for Pair<T> {
    fn printme(&self) -> String {
        let one = self.a.to_string();
        let two = self.b.to_string();
        let mut rtn_string = String::new();
        rtn_string.push_str(&one);
        rtn_string.push_str(&two);
        rtn_string
    }
}

//Conditional implementations

//Implementing external trait on generic type with trait bound syntax
impl<T: std::ops::Add<Output = T> + Copy> Pair<T> {
    pub fn add(&self) -> T {
        self.a + self.b
    }
    //More methods!
}
//External trait on generic type with where clause
impl<T> Pair<T> {
    pub fn addington(&self) -> T
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        self.a + self.b
    }
    //More methods!
}

//==========================================
// Using generic parameters in functions

//Conditional generic function parameters with trait bound syntax
pub fn addotron_1<Q: std::ops::Add<Output = Q>>(a: Q, b: Q) -> Q {
    a + b
}
//Conditional generic function parameters with where clause
pub fn addotron_2<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

pub fn whatever(a: impl PrintMe + std::fmt::Debug, b: impl PrintMe + std::fmt::Debug) {
    println!("{:?} {:?}", a, b)
}

//Example tests!!!
#[test]
fn testing_addotron_2() {
    let result = addotron_2(2, 3);
    assert_eq!(result, 5);
}

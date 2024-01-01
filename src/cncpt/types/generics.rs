#![allow(dead_code)]
#![allow(unused_variables)]

//Use statement necessary for calling code on trait examples
//use crate::cncpt::types::traits::{Summary, NewsArticle, Tweet};

//============================================
//Generics as functions

/**Calling function to illustrate generic function generics_2(); Creates two vectors of different base types and passes them to a function that finds the largest item in the collection;*/
pub fn generics_1() {
    let list_1 = vec![34, 50, 25, 12, 65];
    let processed_1 = generics_5(&list_1);
    println!(
        "The largest via abstraction (and vectors) is {}",
        processed_1
    );
    let list_2 = vec!['d', 'c', 'a', 'b', 'z'];
    let processed_2 = generics_5(&list_2);
    println!(
        "The largest via abstraction (and arrays) is {}",
        processed_2
    );
}

/**Not actually generic; This function takes a reference to a collection and iterates over it to find and return a reference to the largest value in the collection;*/
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
/**This function does the same thing as generics_2() but is defined with generic parameters and a type checker to ensure that only types with the PartialOrd trait are accepted;*/
pub fn generics_5<T>(list: &[T]) -> &T
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
//Generics as structs

/**Defines a generic struct over type T with two fields of the same type (T); When instantiated,
 * both struct fields must be of the same concrete type*/
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl Point<i32> {
    fn get_x(&self) -> &i32 {
        &self.x
    }
}
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
pub fn generics_3() {
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
pub fn generics_4() {
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

#![allow(dead_code)]
#![allow(unused_variables)]

//============================================
//Generics as functions

/**Creates a vector and passes it to a function that finds the largest number in the collection; Two separate functions have been written, one that takes a vector, and one that takes an array; Both work even though the original collection is a vector because the size is known at compile time.*/
pub fn generics_1() {
    let list_1 = vec![34, 50, 25, 100, 65];
    let list_2 = vec![120, 34, 12, 56, 213, 23];
    let list_3 = vec![1, 2, 3, 6, 4];
    let list_4 = vec!['d', 'c', 'a', 'b'];
    let processed_1 = associated_1(&list_1);
    let processed_2 = associated_1(&list_2);
    let processed_3 = associated_1(&list_2);
    println!(
        "The largest via abstraction (and vectors) is {}",
        processed_1
    );
    println!(
        "The largest via abstraction (and arrays) is {}",
        processed_2
    );
}

/**Defines a function as a generic; The function takes a pointer to a vector and iterates over it to find the largest number in the collection; Returns a pointer; Does the same thing as the two previous associated functions but uses a generic type instead of either a vector or array in the function signature (and return value)*/
pub fn associated_1<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        //Our generic type requires proper traits to perform comparisons
        //if item > largest {
        //    largest = item;
        //}
    }
    return largest;
}

//============================================
//Generics as structs

/**Defines a generic struct type*/
struct Point<T> {
    x: T,
    y: T,
}
impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

//Concrete values of two different scalar types used for generic struct
pub fn generics_2() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

#![allow(dead_code)]
#![allow(unused_variables)]

/** Test for a YouTube video that refactors C code*/
pub fn calculate(mut bottom: i32, top: i32) {
    let mut sum = 0;
    let a = bottom;
    let b = top;
    while bottom <= top {
        if bottom % 2 == 0 {
            sum += bottom;
        };
        bottom += 1;
    }
    println!("Adding all the even numbers from {} to {} = {}", a, b, &sum);
}

pub fn calculate_2(bottom: i32, top: i32) -> i32 {
    (bottom..=top).filter(|e| e % 2 == 0).sum()
}

pub fn optional_test() {
    let a = [0i32, 1, 2];
    let b: [i32; 3] = [0, 1, 2];
}

/** Function that tests squares */
pub fn squares_test() {
    let mut x: i32 = 5;
    while x <= 5000000 {
        println!("x = {} x^2 = {}", x, x*x);
        x = x * 10;
    }
}


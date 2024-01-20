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
        println!("x = {} x^2 = {}", x, x * x);
        x = x * 10;
    }
}

/** Converts an i32 to a vector of hex digits */
pub fn hex_digit_finder(mut n: i32) -> Vec<String> {
    let mut r: i32;
    let mut q: i32;
    let mut v: Vec<String> = Vec::new();
    while n > 0 {
        if n <= 16 {
            v.push(n.to_string());
            break;
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

pub fn hex_digit_finder_1(mut n: i32) -> Vec<i32> {
    let mut r: i32;
    let mut q: i32;
    let mut v: Vec<i32> = Vec::new();
    while n > 0 {
        if n <= 16 {
            v.push(n);
            break;
        }
        q = n / 16;
        r = n % 16;
        v.push(r);
        n = q;
    }
    let mut v_index = v.len();
    let mut v_new: Vec<i32> = Vec::new();
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

/** Converts a vector of hex digits to a hex string */
pub fn hex_to_string(v: Vec<i32>) -> String {
    let mut ch: i32;
    let mut s = String::from("0x");
    let mut counter = 0;
    while counter < v.len() {
        ch = v[counter].clone();
        if ch as i32 >= 10 {
            s.push(
                char::from_digit(ch as u32, 16)
                    .unwrap()
                    .to_ascii_uppercase(),
            );
        } else {
            s.push_str(&ch.to_string());
        }
        counter += 1;
    }
    return s;
}

/** Converts an i32 to a hex string */
pub fn number_to_hex(mut n: i32) -> String {
    let mut v: Vec<i32> = Vec::new();
    let mut q: i32;
    let mut r: i32;
    while n > 0 {
        if n <= 16 {
            v.push(n);
            break;
        }
        q = n / 16;
        r = n % 16;
        v.push(r);
        n = q;
    }
    let mut s = String::from("0x");
    for i in v.iter().rev() {
        if *i >= 10 {
            s.push(
                char::from_digit(*i as u32, 16)
                    .unwrap()
                    .to_ascii_uppercase(),
            );
        } else {
            s.push_str(&i.to_string());
        }
    }
    return s;
}

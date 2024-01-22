#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;

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

/** Converts unsigned integers up to 32-bits in size to binary */
pub fn int_to_bin(mut i: i32) -> String {
    let mut q: i32;
    let mut r: i32;
    let mut s = String::new();
    let mut c = 1;
    // Primary logic outputs bits in reverse order
    while i > 0 {
        if i < 2 {
            s.push_str(&i.to_string());
            break;
        }
        r = i % 2;
        if i % 2 > 0 {
            s.push_str("1");
        } else {
            s.push_str("0");
        }
        if c % 4 == 0 {
            s.push_str(" ");
        }
        c += 1;
        i = (i - r) / 2;
    }
    // Reverses the original output to proper order
    //TODO: Rewrite this to push to an array of 0s to provide leading padding to output
    let mut ns = String::new();
    for i in s.chars().rev() {
        ns.push_str(&i.to_string());
    }
    return ns;
}

/** Converts a string to u8 values */
pub fn string_to_val(s: &String) -> Vec<i32> {
    let v: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    for i in v {
        out.push(i as i32);
    }
    return out;
}

/** Converts a vector of 32-bit values and converts them to binary */
pub fn int_to_bin_vec_1(v: Vec<i32>) -> String {
    let mut q: i32;
    let mut r: i32;
    let mut s = String::new();
    // Creates 8-bit bytes representing each character
    let mut byte: Vec<i32> = Vec::new();
    for x in v {
        let mut i = x;
        // Primary logic outputs bits in reverse order
        let mut c = 1;
        while i > 0 {
            if i < 2 {
                byte.push(i);
                s.push_str(&i.to_string());
                break;
            }
            r = i % 2;
            if i % 2 > 0 {
                byte.push(1);
                s.push_str("1");
            } else {
                byte.push(0);
                s.push_str("0");
            }
            //if c % 4 == 0 {
            //    s.push_str(" ");
            //}
            c += 1;
            i = (i - r) / 2;
        }
        s.push_str(" | ");
    }
    // Reverses the original output to proper order
    let mut ns = String::new();
    for i in s.chars().rev() {
        ns.push_str(&i.to_string());
    }
    return ns;
}

/** Takes a vector of char values, converts them to byte vectors,
and pushes each byte value to string; The char type is
4-bytes which is why this takes a vector of i32 */
pub fn int_to_bin_vec_2(v: Vec<i32>) -> String {
    let mut q: i32;
    let mut r: i32;
    let mut s = String::new();
    // Creates 8-bit bytes representing each character
    for x in v {
        let mut i = x;
        let mut byte: Vec<i32> = Vec::new();
        // Primary logic outputs bits in reverse order
        let mut c = 1;
        while i > 0 {
            if i < 2 {
                byte.push(i as i32);
                break;
            }
            r = i as i32 % 2;
            if i % 2 > 0 {
                byte.push(1);
            } else {
                byte.push(0);
            }
            c += 1;
            i = (i - r) / 2;
        }
        // Pads binary values to fill 8-bit byte indexes
        while byte.len() < 8 {
            byte.push(0);
        }
        let mut ordered_byte: Vec<i32> = Vec::new();
        for x in byte.iter().rev() {
            ordered_byte.push(*x);
        }
        //println!("Assembled and ordered byte: {:?}", ordered_byte);
        let joined: String = ordered_byte.iter().map(|&x| x.to_string()).collect();
        s.push_str(&joined);
        s.push_str(" ");
    }
    return s;
}

/** Converts unsigned integers up to 32-bits in size to a hex string */
pub fn int_to_hex(mut n: i32) -> String {
    let mut v: Vec<i32> = Vec::new();
    let mut q: i32;
    let mut r: i32;
    while n > 0 {
        if n < 16 {
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

/** Converts binary to an integer */
pub fn bin_to_str(s: &String) -> u32 {
    let v: Vec<char> = s.chars().collect();
    let mut t: u32 = 0;
    for (i, val) in v.iter().rev().enumerate() {
        if val.to_digit(10).unwrap() == 1 {
            t = t + (2u32.pow(i as u32));
        }
    }
    return t;
}

//TODO: Make this work
//Converts a hex value to a decimal number value */
pub fn hex_to_int(s: String) -> String {
    let mut v: Vec<char> = Vec::new();
    for i in s.chars() {
        v.push(i);
    }
    let mut s = String::new();
    for i in v.iter() {
        if Regex::new(r"^[0-9]+$").unwrap().is_match(&*i.to_string()) {
            s.push_str(&*i.to_string());
        }
    }
    return s;
}

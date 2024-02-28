#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;

#[cfg(test)]
mod tests {
    #[test]
    fn int_to_bin_test() {
        let result = super::int_to_bin(23);
        let expected = String::from("1 0111");
        assert_eq!(result, expected);
        println!("Expected:\n\t{}\nResult\n\t{}", expected, result);
    }

    #[test]
    fn string_to_val_test() {
        let s = String::from("Hello, luv");
        let result = super::string_to_val(&s);
        let expected = vec![72, 101, 108, 108, 111, 44, 32, 108, 117, 118];
        assert_eq!(result, expected);
        println!("Expected:\n\t{:?}\nResult:\n\t{:?}", expected, result);
    }

    #[test]
    fn int_to_bin_vec_test() {
        let v = vec![112, 101, 110, 105, 115];
        let result = super::int_to_bin_vec_2(v);
        let expected = String::from("01110000 01100101 01101110 01101001 01110011 ");
        assert_eq!(result, expected);
        println!("Expected:\n\t{}\nResult\n\t{}", expected, result);
    }

}

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

/** Converts a vector of 32-bit values to binary */
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

/** Converts a vector of i32 values to byte vectors,
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
        match val.to_digit(10) {
            Some(x) => {
                if x == 1 {
                    t = t + (2u32.pow(i as u32));
                }
            },
            None => {
                panic!("Error: char \"{}\" at index {} is not valid binary digit", val, (v.len() - i));
            },
        }
    }
    t

    // Original
    //let v: Vec<char> = s.chars().collect();
    //let mut t: u32 = 0;
    //for (i, val) in v.iter().rev().enumerate() {
    //    if val.to_digit(10).unwrap() == 1 {
    //        t = t + (2u32.pow(i as u32));
    //    }
    //}
    //t

    // Perhaps a more beautiful implmentation, 
    // but does not panic if the String contains
    // a non 1/0 digit
    //let mut t = 0;
    //for (i, val) in s.chars().rev().enumerate() {
    //    if val == '1' {
    //        t |= 1 << i;
    //    }
    //}
    //t
}

#[test]
#[should_panic()]
fn bin_to_str_test() {
    let s = String::from("1001100x0111");
    assert_ne!(bin_to_str(&s), 1223);
}

// Converts a hex value to a decimal number value */
// Precondition: "1F8"
// Should result in: [1, 15, 8]
pub fn hex_to_int(s: &String) -> Vec<u32> {
    let sv: Vec<u32> = s.chars().map(|x| x.to_digit(16).unwrap_or(0)).collect();
    let mut nv: Vec<u32> = Vec::new();
    for i in s.chars() {
        if Regex::new(r"^[0-9]+$").unwrap().is_match(&i.to_string()) {
            nv.push(i.to_digit(10).unwrap());
        } else {
            nv.push(i.to_digit(16).unwrap());
        }
    }
    return nv;
}

/** Converts a hex string into a decimal value */
pub fn hex_to_int_2(s: &String) -> u32 {
    let v: Vec<u32> = s.chars().map(|x| x.to_digit(16).unwrap()).collect();
    let mut t: u32 = 0;
    for (index, value) in v.iter().rev().enumerate() {
        t += value * 16u32.pow(index as u32);
    }
    return t;
}

/** Converts hex to 128-bit decimal value */
fn hex_to_int_3(s: String) -> u128 {
    let v: Vec<u128> = s.chars().map(|x| x.to_digit(16).unwrap() as u128).collect();
    let mut t: u128 = 0;
    for (index, value) in v.iter().rev().enumerate() {
        t += value * 16u128.pow(index as u32);
    }
    t
}


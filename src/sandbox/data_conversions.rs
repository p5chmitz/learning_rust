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

/** Converts a string to u8 values using iterators instead of a for loop */
pub fn string_to_val_2(s: &String) -> Vec<u8> {
    let v: Vec<char> = s.chars().collect();
    let out: Vec<u8> = v.into_iter().map(|x| x as u8).collect();
    out
}

pub fn string_to_val_3(s: &str) -> Box<&[u8]> {
    Box::new(s.as_bytes())
}

/** Converts a vector of 32-bit values to binary */
pub fn int_to_bin_str_1(v: Vec<i32>) -> String {
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
    ns
}

/** Converts a vector of i32 values to byte vectors,
and pushes each byte value to string; The char type is
4-bytes which is why this takes a vector of i32 */
pub fn int_to_bin_str_2(v: Vec<i32>) -> String {
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

/** Converts a String to binary */
pub fn int_to_bin_str_3(s: &str) -> String {
    // Converts the slice to a vector
    let mut v: Vec<i32> = vec![0];
    for c in s.as_bytes() {
        v.push(*c as i32)
    }
    println!("{:?}", v);

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
    s
}
#[test]
fn int_to_bin_str_test() {}

/** Converts unsigned integers up to 32-bits in size to a hex string */
pub fn int_to_hex(mut num: i32) -> String {
    let mut buffer: Vec<i32> = Vec::new();
    let mut quotient: i32;
    let mut remainder: i32;
    while num > 0 {
        if num < 16 {
            buffer.push(num);
            break;
        }
        quotient = num / 16;
        remainder = num % 16;
        buffer.push(remainder);
        num = quotient;
    }
    let mut hex_string = String::from("0x");
    for i in buffer.iter().rev() {
        if *i >= 10 {
            hex_string.push(
                char::from_digit(*i as u32, 16)
                    .unwrap()
                    .to_ascii_uppercase(),
            );
        } else {
            hex_string.push_str(&i.to_string());
        }
    }
    hex_string
}
#[test]
fn int_to_hex_test() {
    assert_eq!(int_to_hex(8079), "0x1F8F".to_string())
}

/** Simplified version */
pub fn int_to_hex_2(int: i32) -> String {
    let mut num = int;
    let mut hex_string = String::from("0x");
    while num > 0 {
        let remainder = num % 16;
        hex_string.insert(
            2,
            match remainder {
                0..=9 => (b'0' + remainder as u8) as char,
                _ => (b'A' + (remainder - 10) as u8) as char,
            },
        );
        num /= 16;
    }
    hex_string
}

/** Converts binary to an integer */
pub fn bin_to_int(s: &String) -> u32 {
    let v: Vec<char> = s.chars().collect();
    let mut t: u32 = 0;
    for (i, val) in v.iter().rev().enumerate() {
        match val.to_digit(10) {
            Some(x) => {
                if x == 1 {
                    t = t + (2u32.pow(i as u32));
                }
            }
            None => {
                println!(
                    "Error: char \"{}\" at index {} is not valid binary digit",
                    val,
                    (v.len() - i)
                );
            }
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

/** Converts a hex value to a byte array of decimal-formatted values;
 * Relies on Regex crate making it inefficient; Also contains two vectors
 * which make it hard to follow and processes the string twice */
pub fn hex_to_int(s: &str) -> Vec<u32> {
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

/** Converts a hex string into a single decimal number value */
pub fn hex_to_int_2(s: &str) -> u32 {
    let vec: Vec<u32> = s.chars().map(|x| x.to_digit(16).unwrap()).collect();
    let mut digit: u32 = 0;
    for (index, value) in vec.iter().rev().enumerate() {
        digit += value * 16u32.pow(index as u32);
    }
    return digit;
}
#[test]
fn hex_to_int_2_test() {
    assert_eq!(80, hex_to_int_2("50"))
}

/** Converts hex to 128-bit decimal value */
fn hex_to_int_3(s: &str) -> u128 {
    let v: Vec<u128> = s.chars().map(|x| x.to_digit(16).unwrap() as u128).collect();
    let mut t: u128 = 0;
    for (index, value) in v.iter().rev().enumerate() {
        t += value * 16u128.pow(index as u32);
    }
    t
}

/** A basic copy of Rust's to_digit method for obtaining the decimal
 * value of a character outside the decimal range; Handles radix values
 * up to 36 with some caveats */
fn hex_to_digit(c: char, radix: u32) -> Option<u32> {
    // Converts Arabic numeral characters to corresponding decimal values
    let mut digit = (c as u32).wrapping_sub('0' as u32);
    // If the value is greater than 10 some actual math happens
    if radix > 10 {
        assert!(radix <= 36, "to_digit: radix is too high (maximum 36)");
        if digit < 10 {
            return Some(digit);
        }
        // Force the 6th bit to be set to ensure ASCII is lower case
        digit = (c as u32 | 0b10_0000)
            .wrapping_sub('a' as u32)
            .saturating_add(10);
    }
    if digit < radix {
        Some(digit)
    } else {
        None
    }
}
#[test]
fn hext_to_digit_test() {
    let t = "1F8".to_string();
    let tv: Vec<u8> = vec![1, 15, 8];
    let mut rv: Vec<u8> = Vec::new();
    for i in t.chars() {
        if let Some(value) = hex_to_digit(i as char, 16) {
            rv.push(value.try_into().unwrap())
        }
    }
    assert_eq!(tv, rv)
    //assert!(
    //    rv == [0], // Incorrect binding causes failure & print
    //    "Test vec: {:?}\nhex_to_int() result: {:?}", tv, rv
    //);
}

fn hex_to_bin(hex_str: &str) -> String {
    let mut s = String::new();
    for i in hex_str.chars() {
        let a = match i.to_digit(16) {
            Some(x) => format!("{:04b}", x),
            None => break,
        };
        s.push_str(&a);
    }
    s
}
#[test]
fn hext_to_bin_test() {
    assert_eq!(hex_to_bin("2A"), "00101010".to_string());
}

fn hex_to_bin_2(hex_str: &str) -> String {
    hex_str
        .as_bytes()
        .iter()
        .flat_map(|&c| match c {
            b'0'..=b'9' => Some((c - b'0') as u32),
            b'A'..=b'F' => Some((c - b'A' + 10) as u32),
            b'a'..=b'f' => Some((c - b'a' + 10) as u32),
            _ => None,
        })
        .map(|x| format!("{:04b}", x))
        .collect()
}
#[test]
fn hext_to_bin_2_test() {
    let hex_value = "2A";
    let binary_value = hex_to_bin_2(hex_value);
    assert_eq!("00101010".to_string(), binary_value);
}

fn hex_to_bin_3(hex_str: &str) -> Box<&[u8]> {
    Box::new(hex_str.as_bytes())
}

fn vec_to_hex() {
    let byte_vector: Vec<u8> = vec![1, 2, 3, 255];
    let hex_string: String = byte_vector
        .iter()
        .map(|byte| format!("{:02x} ", byte))
        .collect();
    println!("Hex representation: {}", hex_string);
}

pub fn data_conversion_driver() {
    // Converts decimal to hex
    println!("\nConverts decimal to hex");
    let v = vec![80, 101, 116, 101, 114];
    for i in v {
        println!("{} to hex: {}", i, int_to_hex(i));
    }

    // Converts Hex to decimal
    println!("\nConverts Hex to decimal");
    let hex = String::from("1F8f");
    println!(
        "{} to byte array in decimal-formatted values: {:?}",
        &hex,
        hex_to_int(&hex)
    );
    println!("{} to decimal: {:?}", &hex, hex_to_int_2(&hex));

    // Converts decimal to binary
    println!("\nConverts decimal to binary");
    let b: i32 = 80;
    let b_array = vec![80, 101, 116, 101, 114];
    for i in b_array.iter() {
        println!("{} to binary: {}", i, int_to_bin(*i))
    }
    println!("{} to binary: {}", b, int_to_bin(b));

    // Converts binary to decimal
    println!("\nConverts binary to decimal");
    let example = String::from("0101100101");
    println!("{} as u32: {}", &example, bin_to_int(&example));
    let example = String::from("1010011010");
    println!("{} as u32: {}", &example, bin_to_int(&example));

    // Converts string to byte array in decimal
    println!("\nConverts strings to byte array formatted as (8-bit) decimal values");
    let s = String::from("Hello");
    println!(
        "\"{}\" as a byte array in decimal format\n\t{:?}",
        &s,
        string_to_val(&s)
    );

    let n = "Peter";
    println!(
        "\"{}\" as a byte array in decimal format\n\t{:?}",
        n,
        n.as_bytes()
    );
    println!("{:?}", "Hell yeah".as_bytes());

    // Converts string to binary
    println!("\nConverts string to binary");
    let p = "Peterman";
    println!("\"{}\" to binary:\n\t{:?}", p, int_to_bin_str_3(p));
    let s = String::from("Murder");
    println!(
        "\"{}\" to binary: \n\t{}",
        &s,
        int_to_bin_str_2(string_to_val(&s))
    );
    let s = "Peter";
    println!(
        "\"{}\" to binary: \n\t{}",
        &s,
        int_to_bin_str_2(string_to_val(&s.to_string()))
    );
    let s = String::from("a8b3d");
    println!(
        "\"{}\" to binary:\n\t{}",
        &s,
        int_to_bin(hex_to_int_2(&s) as i32)
    );
    println!("");

    let s = "5065746572";
    println!("{} as binary is {}", s, hex_to_bin(s));
    println!("{} as binary is {}", s, hex_to_bin_2(s));

    // Converts byte array to Hex format
    println!("Cheap ass vec to hex:");
    vec_to_hex();
}

// TESTS
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
        let result = super::int_to_bin_str_2(v);
        let expected = String::from("01110000 01100101 01101110 01101001 01110011 ");
        assert_eq!(result, expected);
        println!("Expected:\n\t{}\nResult\n\t{}", expected, result);
    }

    #[test]
    fn bin_to_int_test() {
        let s = String::from("1001100x0111");
        assert_ne!(super::bin_to_int(&s), 1223);
    }
}

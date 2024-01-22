#![allow(dead_code)]
#![allow(unused_variables)]
//Re-exported to use in main.rs as cncpt::loops::_my_age_static() for example

pub mod ifs {
    /**Takes a float64 and prints the range*/
    pub fn if_statements(n: f64) {
        if n < 0.33 {
            println!("lower third")
        } else if (0.33 < n) && (n < 0.66) {
            println!("middle third")
        } else if 0.66 < n {
            println!("upper third");
        }
    }
    pub fn if_statements_1(n: f64) -> String {
        let mut result = String::new();
        if n <= 0.0 || n >= 1.0 {
            result.push_str("Out of range; 0.0..1.0");
            return result;
        }
        if n < 0.0 || n > 1.0 {
            panic!("Must be between 0-1")
        }
        if n < 0.33 {
            result.push_str("lower third");
            return result;
        } else if (0.33 < n) && (n < 0.66) {
            result.push_str("middle third");
            return result;
        } else {
            result.push_str("upper third");
            return result;
        }
    }

    #[test]
    fn if_statements_test() {
        let answer = String::from(if_statements_1(1.34));
        assert_eq!("middle third", answer);
    }

    /**Takes a float64 and returns a String range*/
    fn so_many_times(n: f64) -> String {
        let mut s: String = String::new();
        if n < 0.33 {
            s = String::from("lower third");
        } else if (0.33 < n) && (n < 0.66) {
            s = String::from("mids");
        } else if 0.66 < n {
            s = String::from("upper third");
        };
        return s;
    }

    /** Uses an if statement in a declaration.

    `let _i: &str = if n < 5 { "true" } else { "false" };`
     */
    pub fn again_lets_if(n: i32) {
        let _i: &str = if n < 5 { "true" } else { "false" };
        println!("{_i}");
    }
}
pub mod loops {
    /**Introduces the basic loop keyword*/
    fn _loops() {
        let mut i: i32 = 12;
        loop {
            println!("{i}");
            i += 1;
            if i == 23 {
                println!("{i} <- nice");
                i += 1;
                continue;
            }
            if i == 30 {
                println!("The max is: {i}");
                break;
            }
        }
    }

    //Uses a loop in a declaration
    fn _loop_lets() {
        let mut i: i32 = 12;
        let _x: i32 = loop {
            println!("{i}");
            i += 1;
            if i == 23 {
                println!("{i} <- nice");
                i += 1;
                continue;
            }
            if i == 30 {
                break i;
            }
        };
        println!("The max is: {_x}");
    }

    //Uses loop labels for nested/scoped identification
    fn _loop_labels() {
        let mut count: i32 = 1;
        loop {
            println!("{count}");
            let mut second_count: i32 = 101;
            'second_loop: loop {
                println!("{second_count}");
                second_count += 1;
                if second_count == 110 {
                    break 'second_loop;
                }
            }
            count += 1;
            if count == 4 {
                break;
            }
        }
    }

    //While loops, yo
    fn _while_loops() {
        let mut i: i32 = 10;
        while i >= 1 {
            println!("{i}");
            i -= 1;
        }
        println!("Blast off!");
    }

    //Playing around with while loops
    fn _my_age() {
        let mut age: i32 = 0;
        let mut year: i32 = 1983;
        println!("I was born in {}.", &year);
        year += 1;
        while year <= 2023 {
            println!(
                "For most of {} I was {}, but at the very end I turned {}.",
                year,
                age,
                (&age + 1)
            );
            age += 1;
            year += 1
        }
    }

    //Just a static age calculator
    pub fn _my_age_static() {
        let year: i32 = 2023;
        let mut _age: i32 = year - 1984;
        println!(
            "For most of {} I was {}, but at the very end I turned {}.",
            year,
            _age,
            (_age + 1)
        )
    }

    //Just a static age calculator that takes a variable
    fn _my_age_again(year: i32) {
        let mut _age: i32 = year - 1984;
        println!(
            "For most of {} I was {}, but at the very end I turned {}.",
            year,
            _age,
            (_age + 1)
        );
    }

    //For loop with an array
    fn _for_loops() {
        let a: [&str; 4] = ["a", "b", "c", "d"];
        for _index in a {
            println!("{_index}")
        }
        for _i in (1..10).rev() {
            println!("{_i}")
        }
        println!("Blorst off!");
    }
}

fn _main() {}

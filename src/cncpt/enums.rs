//Defines the enum
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
impl IpAddrKind {
    fn access_and_print(a: IpAddrKind) {
        match a {
            IpAddrKind::V4(_a, _b, _c, _d) => {
                println!("The IPv4 address is: {_a}.{_b}.{_c}.{_d}")
            }
            IpAddrKind::V6(_a) => {
                println!("The IPv6 address is: {_a}");
            }
        }
    }
}

enum Message {
    _Quit,
    _Move { _x: i32, _y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}
impl Message {
    fn _set_move(_x: i32, _y: i32) -> Message {
        Message::_Move { _x, _y }
    }
    fn print_message(m: &Message) {
        match m {
            Message::Write(_a) => {
                println!("    auto-print method: {_a}")
            }
            _ => (),
        }
    }
    fn get_message(m: &Message) -> String {
        let mut s: String = String::new();
        match m {
            Message::Write(a) => {
                s = a.to_string();
            }
            _ => (),
        };
        return s;
    }
}

//the coin enum
enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter,
}
impl Coin {
    fn value_of_coin(coin: Coin) -> u8 {
        match coin {
            Coin::_Penny => {
                println!("    that's a penny!");
                1
            }
            Coin::_Nickel => {
                println!("    that's a nickel!");
                5
            }
            Coin::_Dime => {
                println!("    that's a dime!");
                10
            }
            Coin::Quarter => {
                println!("    that's a quarter!");
                25
            }
        }
    }
}

//enum with different structs
enum Movement {
    A(i32, i32),
    //b { x: i32, y: i32, z: i32 },
    C(String),
}
//matches an enum and prints a message
fn testing_types(t: &Movement) -> String {
    let mut _decision: String = Default::default();
    match t {
        Movement::A(..) => _decision = String::from("yes! its enum Movement of type A"),
        //movement::c(..) => _decision = string::from("yes! its the movement type c"),
        _ => _decision = String::from("(type not found)"),
    };
    return _decision;
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        none => none,
        //Some(_i) => Some(_i + 1),
    }
}

fn main() {
    //creates enum instances
    //calls associated function that takes enum types, prints values
    let a1: IpAddrKind = IpAddrKind::V4(10, 0, 0, 1);
    let a2: IpAddrKind = IpAddrKind::V6(String::from("2345:0425:2ca1:0000:0000:0567:5673:23b5"));
    IpAddrKind::access_and_print(a1);
    IpAddrKind::access_and_print(a2);

    //Creates enum instances
    //Prints specific fields from Message using three different techniques
    println!("1) Printing an element from an enum variant with multiple techniques");
    let m: Message = Message::Write(String::from("A rising tide lifts all the homies!"));
    println!("    The message is: {}", Message::get_message(&m));
    Message::print_message(&m);
    //Uses the if let syntax!
    if let Message::Write(_homies) = m {
        println!("    If let syntax message: {_homies}");
    }

    //Writes to enum variant directly and prints variant
    let _mv: Message = Message::_Move { _x: 12, _y: 23 };
    //println!("mva: {:#?}", mv);

    //No such thing as null in Rust
    println!("2) Rust dont have no nulls");
    let _s1: String = String::new();
    println!("    This is a null string: {_s1}");
    let _s2: String = Default::default();
    println!("    This is a default string: {_s2}");

    //Option<T> is a type that must be converted to its base value to be used
    let x: i32 = 12;
    let y: Option<i32> = Some(23);
    let z: i32 = y.unwrap();
    let _xz: i32 = x + z;
    println!("3) The value of x is {x}, and the unwrapped Option<T> type is is: {z}");
    println!("{_xz}");

    //Adds one to the passed number using Option<T>
    let five: Option<i32> = Some(5);
    let none: Option<i32> = None;
    println!("4) More use of the Option<T> type");
    println!("    The value is now: {:?}", plus_one(five));
    println!("    Testing: {:?}", five);
    println!("    The value is now: {:?}", plus_one(Some(11)));
    println!("    The value is now: {:?}", plus_one(none));
    println!("    The value is now: {:?}", plus_one(None));

    //Introduces the match construct
    let input_coin: Coin = Coin::Quarter;
    println!("5) The value of the coin is:");
    println!("{}", Coin::value_of_coin(input_coin));

    //Passes a moment type and returns a string based on a match pattern
    let first: Movement = Movement::A(12, 23);
    let second: Movement = Movement::C(String::from("Hello!"));
    println!("6) Is the input valid??");
    println!("    First test: {}", testing_types(&first));
    println!("    Second test: {}", testing_types(&second));
}

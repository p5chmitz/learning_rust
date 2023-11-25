//This is just how we roll now

//Defines the enum
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
impl IpAddrKind {
    fn access_and_print(a: IpAddrKind) {
        match a {
            IpAddrKind::V4(a, b, c, d) => {
                println!("The IPv4 address is: {a}.{b}.{c}.{d}")
            }
            IpAddrKind::V6(a) => {
                println!("The IPv6 address is: {a}");
            }
        }
    }
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}
impl Message {
    fn set_move(x: i32, y: i32) -> message {
        message::_move { x, y }
    }
    fn print_message(m: &message) {
        match m {
            message::write(a) => {
                println!("    atuo-print method: {a}")
            }
            _ => (),
        }
    }
    fn get_message(m: &message) -> string {
        let mut s: string = string::new();
        match m {
            message::write(a) => {
                s = a.to_string();
            }
            _ => (),
        };
        return s;
    }
}

//the coin enum
enum coin {
    _penny,
    _nickel,
    _dime,
    quarter,
}
fn value_of_coin(coin: coin) -> u8 {
    match coin {
        coin::_penny => {
            println!("    thats a penny!");
            1
        }
        coin::_nickel => {
            println!("    thats a nickel!");
            5
        }
        coin::_dime => {
            println!("    thats a dime!");
            10
        }
        coin::quarter => {
            println!("    thats a quarter!");
            25
        }
    }
}

//enum with different structs
#[derive(debug)]
enum movement {
    a(i32, i32),
    //b { x: i32, y: i32, z: i32 },
    c(string),
}
//matches an enum and prints a message
fn testing_types(t: &movement) -> string {
    let mut _decision: string = default::default();
    match t {
        movement::a(..) => _decision = string::from("yes! its movement type a"),
        //movement::c(..) => _decision = string::from("yes! its the movement type c"),
        _ => _decision = string::from("(type not founderino)"),
    };
    return _decision;
}
fn plus_one(x: option<i32>) -> option<i32> {
    match x {
        none => none,
        some(i) => some(i + 1),
    }
}

fn main() {
    //creates enum instances
    //calls associated function that takes enum types, prints values
    let a1: ipaddrkind = ipaddrkind::v4(10, 0, 0, 1);
    let a2: ipaddrkind = ipaddrkind::v6(string::from("2345:0425:2ca1:0000:0000:0567:5673:23b5"));
    ipaddrkind::access_and_print(a1);
    IpAddrKind::access_and_print(a2);

    //Creates enum instances
    //Prints specific fields from Message using three different techniques
    println!("1) Printing an element from an enum variant with multiple techniques");
    let m: Message = Message::Write(String::from("A rising tide lifts all the homies"));
    println!("    The message is: {}", Message::get_message(&m));
    Message::print_message(&m);
    if let Message::Write(homies) = m {
        println!("    If let syntax message: {homies}");
    }

    //Writes to enum variant directly and prints variant
    let mv: Message = Message::_Move { x: 12, y: 23 };
    //println!("mva: {:#?}", mv);

    //Writes to enum variant and prints variant
    let mva: Message = Message::set_move(12, 1983);
    //println!("mva: {:#?}", mva);
    //println!("IDK WTF: {:#?}", mva.get_move());

    //Introduces the Option<T> enum
    let some_number: Option<i32> = Some(5);
    //dbg!(some_number);
    let some_character: Option<char> = Some('P');
    //dbg!(some_character);
    let absent_value: Option<String> = None;
    //dbg!(absent_value);

    //No such thing as null in Rust
    println!("2) Rust dont have no nulls");
    let s1: String = String::new();
    println!("    This is a null string: {s1}");
    let s2: String = Default::default();
    println!("    This is a default string: {s2}");

    //Option<T> is a type that must be converted to its base value to be used
    let x: i32 = 12;
    let y: Option<i32> = Some(23);
    let z: i32 = y.unwrap();
    let xz: i32 = x + z;
    println!("3) The value of x is {x}, and the unwrapped Option<T> type is is: {z}");

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
    println!("{}", value_of_coin(input_coin));

    //Passes a moement type and returns a string based on a match pattern
    let first: Movement = Movement::A(12, 23);
    let second: Movement = Movement::C(String::from("Hello!"));
    println!("6) Is the input valid??");
    println!("    First test: {}", testing_types(&first));
    println!("    Second test: {}", testing_types(&second));
}

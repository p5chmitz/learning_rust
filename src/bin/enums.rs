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
    Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}
impl Message {
    fn set_move(x: i32, y: i32) -> Message {
        Message::Move { x, y }
    }
    fn print_message(m: &Message) {
        match m {
            Message::Write(a) => {
                println!("    Atuo-print method: {a}")
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

//The Coin enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_of_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("    Thats a penny!");
            1
        }
        Coin::Nickel => {
            println!("    Thats a nickel!");
            5
        }
        Coin::Dime => {
            println!("    Thats a dime!");
            10
        }
        Coin::Quarter => {
            println!("    Thats a quarter!");
            25
        }
    }
}

//Enum with different structs
#[derive(Debug)]
enum Movement {
    A(i32, i32),
    //B { x: i32, y: i32, z: i32 },
    C(String),
}
fn testing_types(t: &Movement) -> String {
    let mut _decision: String = Default::default();
    //Prints a message
    match t {
        Movement::A(..) => _decision = String::from("Yes! Its Movement type A"),
        //Movement::C(..) => _decision = String::from("Yes! Its the Movement type C"),
        _ => _decision = String::from("(Type not founderino)"),
    };
    return _decision;
}
fn my_docs_example(t: &Movement) {
    match t {
        Movement::A(..) => {
            println!("Type A");
        }
        _ => {
            println!("Type not found");
        }
    };
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    //Creates enum instances
    //Calls associated function that takes enum types, prints values
    let a1: IpAddrKind = IpAddrKind::V4(10, 0, 0, 1);
    let a2: IpAddrKind = IpAddrKind::V6(String::from("2345:0425:2CA1:0000:0000:0567:5673:23b5"));
    IpAddrKind::access_and_print(a1);
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
    let mv: Message = Message::Move { x: 12, y: 23 };
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

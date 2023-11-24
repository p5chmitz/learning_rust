//Defines the enum
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
//Function that takes specified enum type
fn route(a: IpAddrKind) {
    println!("The struct is: {:#?}", a)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn set_move(x: i32, y: i32) -> Message {
        Message::Move { x, y }
    }
    fn get_move(&self) -> &Message {
        self
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_of_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Thats a penny!");
            1
        }
        Coin::Nickel => {
            println!("Thats a nickel!");
            5
        }
        Coin::Dime => {
            println!("Thats a dime!");
            10
        }
        Coin::Quarter => {
            println!("Thats a quarter!");
            25
        }
    }
}

//Enum with different structs
#[derive(Debug)]
enum Movement {
    A(i32, i32),
    B { x: i32, y: i32, z: i32 },
    C(String),
}

fn main() {
    //Creates enum instances
    let addr1: IpAddrKind = IpAddrKind::V4(10, 0, 0, 1);
    let addr2: IpAddrKind = IpAddrKind::V6(String::from("2345:0425:2CA1:0000:0000:0567:5673:23b5"));
    println!("{:?}", addr1);
    //Calls function that takes enum types
    route(addr2);

    //Writes to field(s) and prints variant
    let m: Message = Message::Write(String::from("A rising tide lifts all the homies"));
    println!("m: {:#?}", m);
    let cc: Message = Message::ChangeColor(12, 23, 1983);
    println!("cc: {:#?}", cc);

    //Writes to enum variant directly and prints variant
    let mv: Message = Message::Move { x: 12, y: 23 };
    println!("mva: {:#?}", mv);

    //Writes to enum variant and prints variant
    let mva: Message = Message::set_move(12, 1983);
    println!("mva: {:#?}", mva);
    println!("IDK WTF: {:#?}", mva.get_move());

    //Introduces the Option<T> enum
    let some_number: Option<i32> = Some(5);
    dbg!(some_number);
    let some_character: Option<char> = Some('P');
    dbg!(some_character);
    let absent_value: Option<String> = None;
    dbg!(absent_value);

    //No such thing as null in Rust
    let stringa: String = String::new();
    println!("This is a null string: {stringa}");
    let s: String = Default::default();
    println!("This is a default string: {s}");

    let x: i32 = 12;
    let y: Option<i32> = Some(23);
    let z: i32 = y.unwrap();
    let xz: i32 = x + z;
    println!("The value of the unwrapped value is: {xz}");

    //Introduces the match construct
    let input_coin: Coin = Coin::Quarter;
    println!("The value of the coin is: {}", value_of_coin(input_coin));

    
}

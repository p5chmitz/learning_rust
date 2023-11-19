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
struct TestingEnumsStruct {
    one: String,
    two: i32,
}
#[derive(Debug)]
enum TestingEnumsEnum {
    A(TestingEnumsStruct),
    B(TestingEnumsStruct, String),
}
fn main() {
    //Builds enum instances
    let addr1: IpAddrKind = IpAddrKind::V4(10, 0, 0, 1);
    let addr2: IpAddrKind = IpAddrKind::V6(String::from("2345:0425:2CA1:0000:0000:0567:5673:23b5"));

    //Calls function that takes enum types
    route(addr2);

    //Builds struct
    let first: TestingEnumsStruct = TestingEnumsStruct {
        one: String::from("Peter is definitely a person"),
        two: 0,
    };
    let second: TestingEnumsStruct = TestingEnumsStruct {
        one: String::from("Hello, world"),
        two: 23,
    };
    let third = TestingEnumsEnum::A(first);
    println!("{:#?}", third);
    //println!("Greeting: {}", &first.one)
}

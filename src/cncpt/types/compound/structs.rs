#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub struct User {
    struct_name: String,
    active: bool,
    username: String,
    email: String,
    _sign_in_count: i32,
}
impl User {
    /**It is also possible to use the "Field Init Shorthand" for self-same fields.*/
    pub fn constructor(struct_name: String, email: String, username: String) -> User {
        User {
            struct_name,
            active: false,
            username,
            email,
            _sign_in_count: 1,
        } //Implicitly returns User
    }
    /**User struct method that prints the name field*/
    pub fn print_me(&self) {
        println!("ID: {}", self.struct_name);
        println!("Username: {}", self.username);
        println!("Email: {}", self.email);
        println!("Active: {}", self.active);
    }
    /** Requires a mutable argument*/
    pub fn is_active(&mut self, b: bool) {
        self.active = b;
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Uses two unrelated arguments to make a calculation
fn area(w: u32, h: u32) -> u32 {
    w * h
}
//Uses one argument of type tuple but we dont know how the elements relate
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
//Takes a single, (immutable) borrow argument of a Rectangle instance (&rect2)
fn area2(r: &Rectangle) -> u32 {
    &r.width * &r.height
}
impl Rectangle {
    //Implements the area method
    fn constructor(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
    fn area3(self: &Self) -> u32 {
        &self.width * &self.height
    }
    //Implements the can_hold method
    fn is_wider(&self, second_instance: &Rectangle) -> bool {
        self.width > second_instance.width
    }
    //Implements the fit_inside method
    fn fit_inside(&self, second_instance: &Rectangle) -> bool {
        self.width <= second_instance.width && self.height <= second_instance.height
    }
    //Mutates the size of the rectangle
    fn mutate(&self, x: u32, y: u32) -> u32 {
        (&self.width * x) * (&self.height * y)
    }
}

pub fn struct_demo_1() {
    let mut user = User::constructor(
        String::from("0001"),
        String::from("me@mine.com"),
        String::from("pschmitz"),
    );
    user.print_me();
    println!("===========");
    user.is_active(true);
    user.print_me();
}

fn main() {
    //================================================
    //The User struct
    //Manually instantiates/builds the User struct as user1
    //Notice that the struct is immutable
    let user1 = User {
        struct_name: String::from("user1"),
        active: true,
        username: String::from("Peter"),
        email: String::from("peter@email.com"),
        _sign_in_count: 23,
    };
    //Creates immutable borrow of instantiated struct
    let user_name: &String = &user1.username;
    //Instantiates a second struct by using the struct update syntax
    //Moves the values from the first struct instance to a second struct
    //Invalidates the user1 struct
    let user2 = User {
        struct_name: String::from("user2"),
        username: String::from("Michael"),
        ..user1
    };
    //Prints the user2 struct; The user1 struct is invalidated by the (partial) move
    //user1.print_me(); //Now illegal due to partial move
    user2.print_me();

    //Instantiates a new User struct using a constructor/build function
    //Uses an immutable borrow to set the name of the struct
    let new_struct_name: String = String::from("user3");
    let new_struct: User = User::constructor(
        String::from(&new_struct_name),
        String::from("dingus@dorkus.com"),
        String::from("dangus"),
    );
    println!("{}", new_struct_name);
    new_struct.print_me();

    //================================================
    //Tuple structs are weird
    struct Green(i32, i32, i32);
    let _color_name: String = String::from("Green");
    let green = Green(134, 187, 140);
    let _v1 = green.0;
    let _v2 = green.1;
    let _v3 = green.2;
    println!("The RGB value of {_color_name} is set to: {_v1}, {_v2}, {_v3}");

    //================================================
    //The Rectangle struct
    //Instantiates a Rectangle struct via constructor
    let first = Rectangle::constructor(20, 30);
    //Uses Rectangle method to calculate and print a value
    println!("The area of rekt is: {}", first.area3());

    //Basic functions for illustration
    let width: u32 = 30;
    let height: u32 = 50;
    println!(
        "1) The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    let rect1: (u32, u32) = (60, 50);
    println!(
        "2) The area of the rect1 is {} square pixels.",
        area1(rect1)
    );

    //Instantiates a Rectangle and passes an immutable borrow to print the area
    let rect2 = Rectangle {
        width: 90,
        height: 50,
    };
    println!(
        "3) The area of the rect2 is {} square pixels.",
        area2(&rect2)
    );

    //Instantiates another Rectangle to compare to the first using immutable borrows passed to
    //implemented methods
    let rect3 = Rectangle {
        width: 120,
        height: 50,
    };
    println!(
        "4) The width of rect2 is {} and the width of rect3 is {}.",
        &rect2.width, &rect3.width
    );
    println!(
        "    Is rect2 wider than rect3? The answer is: {}.",
        rect2.is_wider(&rect3)
    );
    println!(
        "    Does rect2 fit inside rect3? The answer is: {}",
        &rect2.fit_inside(&rect3)
    );
    println!(
        "    If we multiply the width 2x and the height 3x the new area is {}",
        &rect2.mutate(2, 3)
    );
    println!(
        "5) The area of the rect3 is {} square pixels.",
        rect3.area3()
    );

    //================================================
    //Printing the struct via debug trait
    //The struct requires the #[derive(Debug)] annotation for this to work!
    //The # symbol in the format specifier is something like a format prettier
    let rekt = Rectangle {
        width: 12,
        height: 23,
    };
    //Uses derive attribute with Debug trait and :#?
    println!("The struct \"rekt\" is: {:#?}", rekt);
    dbg!(rekt);
}

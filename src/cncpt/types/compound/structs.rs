#[derive(Debug)]
struct User {
    struct_name: String,
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: i32,
}
impl User {
    fn print_me(&self) {
        println!("{}", self.struct_name)
    }
}

//It is also possible to use the "Field Init Shorthand" for self-same fields.
fn build_user(struct_name: String, email: String, username: String) -> User {
    User {
        struct_name,
        _active: true,
        username,
        email,
        _sign_in_count: 1,
    } //Implicitly returns User
}
//Prints the struct
fn print_struct_with_name(new_struct_name: String, _user_email: &String, _user_name: &String) {
    println!("Struct: {}", &new_struct_name);
    println!("Email: {_user_email}");
    println!("Username: {_user_name}");
    println!();
}
fn print_struct(_user_email: &String, _user_name: &String) {
    println!("Struct: [generic]");
    println!("Email: {_user_email}");
    println!("Username: {_user_name}");
    println!();
}

fn another_main() {
    //Instantiates/builds the User struct as user1
    let user_name: String = String::from("user1");
    let user1: User = User {
        struct_name: user_name,
        _active: true,
        username: String::from("peter"),
        email: String::from("peter@email.com"),
        _sign_in_count: 23,
    };
    //Accesses struct value(s) in the user1 instance
    let mut user_email: &String = &user1.email;
    let user_name: &String = &user1.username;
    print_struct(&user_email, &user_name);
    //    println!("Debugging: {:#?}", user1);
    dbg!(&user1);
    println!();

    //Resets the value of username from the mutable struct in the user1 instance
    let user_name: &String = &String::from("petername");
    print_struct(&user_email, &user_name);

    //Uses the struct update syntax to copy values from user1 instance
    let user2: User = User {
        struct_name: String::from("user2"),
        username: String::from("Michael"),
        ..user1
    };
    //print_struct_with_name(user2.struct_name, &user2.email, &user2.username);
    user2.print_me();

    //Instantiates a new User struct using a build function
    let new_struct: String = String::from("user3"); //Names struct
    let new_struct_name: String = String::from(&new_struct); //Creates new String from borrowed name
                                                             //Builds new struct with values
    let new_struct: User = build_user(
        String::from(&new_struct_name),
        String::from("dingus@dorkus.com"),
        String::from("dangus"),
    );
    user_email = &new_struct.email;
    let user_name: &String = &new_struct.username;
    print_struct_with_name(new_struct_name, &user_email, &user_name);

    //Tuple structs are weird
    struct Green(i32, i32, i32);
    let _color_name: String = String::from("Green");
    let green = Green(134, 187, 140);
    let _v1 = green.0;
    let _v2 = green.1;
    let _v3 = green.2;
    println!("The RGB value of {_color_name} is set to: {_v1}, {_v2}, {_v3}");
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
    fn _whatever() -> String {
        String::from("Is this a method?")
    }
    fn constructor(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
}

fn main() {
    let first: Rectangle = Rectangle::constructor(20, 30);
    println!("The area of rekt is: {}", first.area3());

    let width: u32 = 30;
    let height: u32 = 50;
    let rect1: (u32, u32) = (60, 50);
    let rect2 = Rectangle {
        width: 90,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 120,
        height: 50,
    };

    println!(
        "1) The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    println!(
        "2) The area of the rect1 is {} square pixels.",
        area1(rect1)
    );
    println!(
        "3) The area of the rect2 is {} square pixels.",
        area2(&rect2)
    );
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

    //Printing the struct
    let rekt = Rectangle {
        width: 12,
        height: 23,
    };
    //Uses derive attribute with Debug trait and :#?
    println!("The struct \"rekt\" is: {:#?}", rekt);
}

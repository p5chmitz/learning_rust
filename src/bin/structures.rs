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

fn main() {
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

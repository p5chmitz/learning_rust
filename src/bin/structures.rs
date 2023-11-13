struct User {
    struct_name: String,
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32,
}

//It is also possible to use the "Field Init Shorthand" for self-same fields.
fn build_user(struct_name: String, email: String, username: String) -> User {
    User {
        struct_name: struct_name,
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    } //Implicitly returns User
}
//Prints the struct
fn print_struct_with_name(new_struct_name: String, uemail: &String, uname: &String) {
    println!("Struct: {}", &new_struct_name);
    println!("Email: {uemail}");
    println!("Username: {uname}");
    println!("");
}
fn print_struct(uemail: &String, uname: &String) {
    println!("Struct: [generic]");
    println!("Email: {uemail}");
    println!("Username: {uname}");
    println!("");
}

fn main() {
    //Instantiates/builds the User struct as user1
    let uname: String = String::from("user1");
    let mut user1: User = User {
        struct_name: uname,
        active: true,
        username: String::from("peter"),
        email: String::from("peter@email.com"),
        sign_in_count: 23,
    };
    //Accesses struct value(s) in the user1 instance
    let mut uemail: &String = &user1.email;
    let mut uname: &String = &user1.username;
    print_struct(&uemail, &uname);

    //Resets the value of username from the mutable struct in the user1 instance
    let uname: &String = &String::from("petername");
    print_struct(&uemail, &uname);

    //Uses the struct update syntax to copy values from user1 instance
    let mut user2: User = User {
        struct_name: String::from("user2"),
        username: String::from("Michael"),
        ..user1
    };
    print_struct_with_name(user2.struct_name, &user2.email, &user2.username);

    //Instantiates a new User struct using a build function
    let new_struct: String = String::from("user3"); //Names struct
    let new_struct_name: String = String::from(&new_struct); //Creates new String from borrowed name
                                                             //Builds new struct with values
    let new_struct: User = build_user(
        String::from(&new_struct_name),
        String::from("dingus@dorkus.com"),
        String::from("dangus"),
    );
    uemail = &new_struct.email;
    let uname: &String = &new_struct.username;
    print_struct_with_name(new_struct_name, &uemail, &uname);

    //Tuple structs are weird
    struct Green(i32, i32, i32);
    let color_name: String = String::from("Green");
    let green = Green(134, 187, 140);
    let v1 = green.0;
    let v2 = green.1;
    let v3 = green.2;
    println!(
        "The RGB value of {} is set to: {v1}, {v2}, {v3}",
        color_name
    );

}

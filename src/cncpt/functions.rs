//Function that returns a value
fn return_type() -> i32 {
    let x: i32 = 12;
    let y: i32 = 23;
    x + y
}
//Another function that returns a value
fn return_type_two() -> i32 {
    let x: i32 = 23;
    let y: i32 = 83;
    return x + y;
}
//Exploring return statements
fn working_with_return_statements() {
    println!("{} | {}", return_type(), return_type_two());
}

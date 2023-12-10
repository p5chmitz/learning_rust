pub fn vec_test_1() {
    //Adds type annotation because no type is provided for inference
    let mut v: Vec<String> = Vec::new();
    let bday: String = String::from("12/23/1983");
    let intro: String = String::from("My birthday is: ");
    v.push(intro);
    v.push(bday);
    println!("{}{}", v.get(0).unwrap(), v.get(1).unwrap());
    //println!("My birthday is: {}", v.get(n).unwrap());
}

pub fn vec_test_2(mut i: i32) {
    //let mut v: Vec<i32> = Vec::with_capacity(i as usize);
    let mut v: Vec<i32> = Vec::new();
    println!("i is: {i}");
    while i >= 1 {
        v.push(i.try_into().unwrap());
        i -= 1;
    }
    println!("i is: {i}");
    for x in &v {
        println!("{x}");
        match x {
            1 => {
                println!("Blastoff!");
            }
            _ => {}
        }
    }
    println!("i is: {i}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Uses two unrelated arguments to make a calculation
fn area(w: i32, h: i32) -> i32 {
    w * h
}
//Uses one argument of type tuple but we dont know how the elements relate
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
//Takes a single, (immutable) borrow argument of a Rectangle instance (&rect2)
fn area2(r: &Rectangle) -> u32 {
    r.width * r.height
}
//Implements the area method
impl Rectangle {
    fn area3(self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width = 30;
    let height = 50;
    let rect1 = (60, 50);
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
        "2) The area of the rectangle is {} square pixels.",
        area1(rect1)
    );
    println!(
        "3) The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );
    println!(
        "The values of the rectangle are width: {} and height: {}",
        &rect2.width, &rect2.height
    );
    println!(
        "4) The area of the rectangle is {} square pixels.",
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

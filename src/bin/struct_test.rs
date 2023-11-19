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
    r.width * r.height
}
impl Rectangle {
    //Implements the area method
    fn area3(self: &Self) -> u32 {
        self.width * self.height
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
        (self.width * x) * (self.height * y)
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

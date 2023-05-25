#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );


    let rect1: (u32, u32) = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    let rect2 = Rectangle {
        width: 30, 
        height: 50,
    };

    println!("rect2 is {:?}", rect2);

    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels!!",
        rect2.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 200,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 100,
    };

    let rect3 = Rectangle {
        width: 150,
        height: 250,
    };

    let rect4 = Rectangle {
        width: 1000,
        height: 50,
    };

    println!("The area of triangle is {} square pixels", rect1.area());
    println!("The rectangle has nonzero width; it is {}", rect1.width);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

    let square = Rectangle::square(5);
}

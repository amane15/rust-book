#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("The area of rectangle is {} square pixels", area(&rect1));
    println!("react1 is {:#?}", rect1);
    dbg!(&rect1);
}

// Accessing fields of a borrowed struct instance does not move the field values,
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

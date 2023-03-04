#![allow(unused_variables)]
#![allow(dead_code)]

use std::cmp::PartialOrd;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![10, 1, 11, 23, 45];
    let result = largest(&number_list);
    println!("The largest is {}", result);

    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 23.4, y: 54.5 };
    println!("integer.x = {}", integer.x());
}

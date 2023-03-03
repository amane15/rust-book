pub mod garden; // line tells the compiler to include the code it finds in src/garden.rs

use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}

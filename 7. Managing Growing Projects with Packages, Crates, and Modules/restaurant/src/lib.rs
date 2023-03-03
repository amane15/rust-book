#![allow(dead_code)]
#![allow(unused_imports)]

// The module tree should be defined in src/lib.rs.
use std::io::{self, Write}; // bring both io and io::write into scope
use std::{cmp::Ordering, io::Read};
// the glob operator
use std::collections::*; // bring all public items into scope

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}
// We can construct relative paths that begin in the parent module,
// rather than the current module or the crate root, by using super at
// the start of the path.

mod back_of_house {
    // if we make an enum public, all of its variants are then
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("grapes"),
            }
        }
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
}

// In Rust, all items (functions, methods, structs, enums, modules, and
// constants) are private to parent modules by default.

// use only creates the shortcut for the particular scope in which the use occurs.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
}

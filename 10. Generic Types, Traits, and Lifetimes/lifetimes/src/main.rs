#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        1
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ismael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime
    // Static lifetime denotes that each affected reference can live
    // for the entire duration of the program
    // All string literals have static lifetime
    let s: &'static str = "I have static lifetime";
}

/*
 * The function signature now tells Rust that for some lifetime 'a,
 * the function takes two parameters, both of which are string slices
 * that live at least as long as lifetime 'a. The function signature also
 * tells Rust that the string slice returned from the function will live
 * at least as long as lifetime 'a. In practice, it means that the
 * lifetime of the reference returned by the longest function is the
 * same as the smaller of the lifetimes of the values referred to by
 * the function arguments. These relationships are what we want Rust
 * to use when analyzing this code.
 */
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

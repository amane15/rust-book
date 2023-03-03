#![allow(unused_variables)]

fn main() {
    let data = "initial contents";

    let mut s = data.to_string();

    s.push_str("string");

    // concatenation with + operator
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // value of s1 is moved
    println!("s3 is {s3}");

    // format!() macro
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = format!("{tic}-{tac}-{toe}");
    println!("{tic_tac_toe}");

    // slicing strings
    let hello = "Здравствуйте";
    let hello_slice = &hello[0..4];

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}

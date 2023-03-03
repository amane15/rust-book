#![allow(unused_variables)]
#![allow(dead_code)]

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap
    // unwrap will return the value inside Ok if value is the Ok variant
    // or else it will call panic! macro
    let greeting_file2 = File::open("hello.txt").unwrap();

    // expect
    let greeting_file3 =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // ? operator: shortcut for propogating errors
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
    // The return type of the function has to be a Result so that
    // it’s compatible with this return.
}
/*
 * Error values that have the ? operator called on them go through the
 * from function, defined in the From trait in the standard library,
 * which is used to convert values from one type into another.
 * When the ? operator calls the from function, the error type received
 * is converted into the error type defined in the return type of the
 * current function. This is useful when a function returns one error
 * type to represent all the ways a function might fail, even if parts
 * might fail for many different reasons.
*/

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

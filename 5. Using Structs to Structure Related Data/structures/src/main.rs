#![allow(dead_code)]
#![allow(unused_variables)]

// Defining a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like structs
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user@email.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    // To change struct fields entire struct should be mutable
    let mut user2 = User {
        active: false,
        email: String::from("user2@email.com"),
        username: String::from("user2"),
        sign_in_count: 2,
    };

    user2.email = String::from("test@email.com");
    user2.username = String::from("user2");

    println!("{}", user2.email);

    // Creating instance from other instances with struct update syntax
    let user3 = User {
        email: user2.email,
        username: user2.username,
        active: true,
        sign_in_count: user2.sign_in_count,
    };
    // Or
    let user4 = User {
        email: String::from("user4@email.com"),
        ..user1
    };

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like struct
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

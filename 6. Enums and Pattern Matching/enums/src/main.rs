#![allow(dead_code)]
#![allow(unused_variables)]

/*
 * The name of each enum variant that we define also becomes a function
 * that constructs an instance of the enum. That is, IpAddr::V4() is a
 * function call that takes a String argument and returns an instance of
 * the IpAddr type. We automatically get this constructor function
 * defined as a result of defining the enum.
 */
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let msg = Message::Write(String::from("hello"));
    msg.call();

    let some_number = Some(4);
    let some_char = Some('c');
    let absent_number: Option<i32> = None; // type annotation is required
}

fn route(ip_kind: IpAddrKind) {}

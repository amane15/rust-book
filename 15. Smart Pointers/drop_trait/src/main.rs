#![allow(unused_variables)]

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let pointer1 = CustomSmartPointer {
        data: String::from("Pointer 1"),
    };
    let pointer2 = CustomSmartPointer {
        data: String::from("Pointer 2"),
    };
    println!("CustomSmartPointers created");
    // std::mem::drop
    drop(pointer1);
    println!("CustomSmartPointer dropped before the end of main");
}

#![allow(dead_code)]
#![allow(unused_variables)]

enum SpreadSheetCell {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5];

    let mut v1 = vec![1, 2, 3];
    v1.push(4);

    let second: &i32 = &v[1];
    println!("Second element is {}", second);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // iterating over the values in a vector
    for i in &v {
        println!("{i}");
    }

    let mut mut_v = vec![1, 2, 3, 4, 5];
    for i in &mut mut_v {
        *i += 10;
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(5.3),
        SpreadSheetCell::Text(String::from("blue")),
    ];
}

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    // iterating over a hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // entry method
    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("red")).or_insert(20);

    println!("{:?}", scores);

    // updating value based on old value
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

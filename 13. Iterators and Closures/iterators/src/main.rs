fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    let v1_iter = v1.iter();
    let total: u32 = v1_iter.sum();
    assert_eq!(total, 6);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

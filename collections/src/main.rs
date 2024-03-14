fn main() {
    // Vectors

    let _v1: Vec<i32> = Vec::new();

    let _v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];

    let third = &v4[2];
    println!("The third element is {}", third);

    let fifth: Option<&i32> = v4.get(5);

    match fifth {
        Some(fifth) => println!("The fifth element is {}", fifth),
        None => println!("There is no fifth element"),
    }

    let v6 = vec![100, 32, 57];

    for i in &v6 {
        println!("{i}!")
    }

    let mut v7 = vec![100, 32, 57];

    for i in &mut v7 {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Hashmaps

    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Blue")).or_insert(60);
    scores.entry(String::from("Orange")).or_insert(60);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:?}", map);

}

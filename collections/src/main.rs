use std::char;

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

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for item in row {
        println!("{item:?}")
    }

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
    
    // Exercises

    // Exercise 1
    let integers1 = vec![4, 6, 7, 2, 5, 6, 5, 2, 5, 5, 6];
    let integers2 = vec![9, 2, 6, 6, 9, 1, 3, 1, 7, 1];

    fn get_median(vector: &Vec<i32>) -> i32 {
        let vector = vector.clone();
        let length = vector.len();
        
        if length % 2 == 0 {
            let median_entries = (length / 2, (length / 2) - 1);

            vector[median_entries.0] + vector[median_entries.1] / 2

        } else {
            let median_entry = (length - 1) / 2;
            vector[median_entry]
        }
    }

    let median1 = get_median(&integers1);
    let median2 = get_median(&integers2);

    println!("The medians are {median1}, {median2}");

    fn get_mode(vector: &Vec<i32>) -> i32 {
        let mut map:HashMap<i32, i32> = HashMap::new();
        
        for item in vector {
            let count = map.entry(*item).or_insert(0);
            *count += 1;
        }

        // mode(item, count)
        let mut mode = (0, 0);

        for (item, count) in map {
            if count > mode.1 {
                mode = (item, count)
            }
        }

        mode.0
    }

    let mode1 = get_mode(&integers1);
    let mode2 = get_mode(&integers2);

    println!("The modes are {mode1}, {mode2}");

}

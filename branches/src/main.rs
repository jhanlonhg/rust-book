fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true!")
    } else {
        println!("Condition was false!")
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
    
        if counter == 10 {
            break counter * 2
        }
    };

   println!("The result is {result}");
}

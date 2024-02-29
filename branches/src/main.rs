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
            break counter * 2 // Can return a value from a break!
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;
        println!("count = {count}");

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up; // Can break an outer loop from an inner loop!
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("Final count = {count}");
}

use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    // -- Panic on error --

    // let greeting_file = File::open("hello.txt");

    // match greeting_file {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening file: {error:?}"),
    // };

    // -- Match to create file or report on errors --

    // let greeting_file_result = File::open("hello.txt");

    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Error opening file: {other_error:?}");
    //         }
    //     },
    // };

    // -- Unwrap calls panic --

    // let _greeting_file = File::open("hello.txt").unwrap();

    // -- Expect calls panic with custom error message --

    // let _greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    // -- Propagating errors --

    // -- Long form propagation --
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");

    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }

    // -- Short form propagation -- 
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }

    // -- Shorter form propagation -- 
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username = String::new();

    //     File::open("hello.txt")?.read_to_string(&mut username)?;

    //     Ok(username)
    // }

    // -- Shortest form propagation --
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     fs::read_to_string("hello.txt")
    // }

    // let _username = match read_username_from_file() {
    //     Ok(username) => username,
    //     Err(e) => panic!("Error retrieving username from file: {e:?}"),
    // };

} 

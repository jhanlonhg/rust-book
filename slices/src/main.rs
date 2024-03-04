fn main() {
    let my_string_literal = "Hello world";
    let my_string = String::from(my_string_literal);

    // Passes in full string as a reference
    let _word = first_word(&my_string);

    // Passes in full string as an explicit string slice
    let _word = first_word(&my_string[..]);

    // Passing in a string literal as it exists as a string reference under the hood
    let _word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i] // return a slice of the string from byte 0 to i  
        }
    };

    &s[..] // return the entire string as no space was found 
}
// A slice is a type of reference - does not have ownership
fn main() {
    let s = String::from("hello world");

    // Slice is created by specifying a range within brackets
    // Slice range indicies must occur at valid UTF-8 char boundaries
    let hello = &s[0..5];   // &s[..5] --if the slice starts at 0
    let world = &s[6..11];  // &s[6..] --if the slice goes until the end of the String
    let hello_world = &s[..];   // [..] takes a slice of the entire string

    let word = first_word(&s);
    
    println!("The first word in 'hello world' is: {word}");
    
    // Array slice example
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let a_slice = &a[1..3];
}

// &str parameter allows use of first_word() on both str and String values
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
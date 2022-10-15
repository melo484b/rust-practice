use std::{collections::HashMap, hash::Hash};

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
// When a vector is dropped, all of its elements are dropped and cleaned up
fn main() {
    let v = vec![55, 42, 86, 987, 4];

    let third: &i32 = &v[2];

    // Cannot push to a vector while a reference is held to an element as it may cause 
    // memory to be reallocated
    // v.push(99);

    println!("The third number in the list is: {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third number in the list is: {third}"),
        None => println!("There is no third element."),
    }

    // Will cause a panic by attempting to access an element outside the avialable indexes
    // let does_not_exist = &v[100];

    // Returns None without panicking
    let does_exist = v.get(100);

    // Iterating through a vector
    for i in &v {
        println!("{}", i)
    }

    let mut v2 = vec![100, 45, 87];
    for i in &mut v2 {
        // Dereference i so it can be modified at its memory location
        *i += 30;
        println!("{}", i);
    }

    // All enum variants are considered to be the same type
    // Vectors can only store data of the same type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings
    // str (string slice) is part of Rust core & references UTF-8 string data stored elsewhere
    // String (string literal) is part of Rust std library and is stored in the program's binary
    let data = "initial contents";

    // .to_string() can be called on any type that implements the Display trait 
    // let s = data.to_string();

    // .to_string() called on a string literal
    // let s2 = "initial contents".to_string();

    // Updating strings
    let mut s3 = String::from("foo");
    // push_str method takes a string slice, avoids taking ownership of the parameter
    s3.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    // .push() appends a chracter to the String
    s.push('l');

    // Concatonation with + or format!
    let hello = String::from("Hello");
    let world = String::from(" world");
    // The String from hello is moved into hello_world and is no longer valid
    let hello_world = hello + &world;

    // In the example above, &world is changed into a &world[..] by deref coercion
    // + operater uses add() from the standard library
    // add() takes ownership of self (hello in this example), but does not take ownership of world
    // a copy of the contents of world is appended to hello
    // the ownership of the result is returned

    // Concatonate multiple strings
    let s5 = String::from("tic");
    let s6 = String::from("tac");
    let s7 = String::from("toe");

    // + operator method
    // let s8 = s5 + "-" + &s6 + "-" + &s7;

    // Using !format - !format does not take ownership of any of its parameters
    let s8 = format!("{}-{}-{}", s5, s6, s7);

    // Indexing strings - Rust strings do not support indexing i.e. hello_world[0]
    // Strings are implemented as a wrapper over vec<u8>
    // Strings from Rust's perspective: bytes, scalar values, and grapheme clusters

    // String slices can be used to create slices of certain bytes
    let s8_slice = &hello_world[0..4];

    // Iterating over Strings - be specific
    // Note: Valid unicode scalar values may be made up of more than 1 byte
    for c in "hi".chars() {
        println!("{}", c);
    }

    for b in "hi".bytes() {
        println!("{}", b);
    }

    // Hash Maps - data stored on the heap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 55);
    scores.insert(String::from("Yellow"), 22);

    // Access values in a HashMap
    let team_name = String::from("Blue");
    // .get() returns an Option<&V> if there is no value for the key, it returns None
    // .copied() returns an Option<i32> 
    // .unwrap_or() sets score to 0 if there is no value associated with the key
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterate over HashMap
    // The pairs are retrieved in an arbitrary order
    // for (key, value) in scores {
    //     println!("{}: {}", key, value)
    // }

    // HashMap ownership - types that implement the Copy trait are copied into the hash map
    // owned values like Strings will be moved into the hash map

    // After being inserted into the hash map with .insert() these Strings are no longer valid
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Updating hash maps - overwrite, add if key not present, update based on old value
    // Overwriting
    scores.insert(String::from("Yellow"), 40);
    println!("{:?}", scores);

    // Add a key-value pair if the key is not present
    // .entry() returns an enum Entry 
    // .or_insert() returns a mutable reference to the value for the corresponding entry if it exists
    // if it doesn't, the parameter is used as the new value for the key
    scores.entry(String::from("Yellow")).or_insert(50);

    // Update a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // If the map does not contain the word, the count is set to 0
        let count = map.entry(word).or_insert(0);
        // Mutable reference to the value is stored in count - is dereferenced to allow assignment
        *count += 1;
    }

    println!("{:?}", map);

}

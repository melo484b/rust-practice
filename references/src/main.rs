fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);
//  let (s2, len) = calculate_length(s1);

//  println!("The length of '{}' is {}.", s2, len);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable variable
    let mut s3 = String::from("I'm mutable");

    let r1 = &mut s3;
    {
        // The references can be used in different scopes
        let r2 = &mut s3;
    }

    // Multiple references to the same data (at the same time) is not allowed
    // println!("{}, {}", r1, r2);

    let r3 = &s3;
    let r4 = &s3;
    println!("{}, {}", r3, r4);

    let r5 = &mut s3;
    // Nor can &mut and & references be combined
    // println!("{}, {}, {}", r3, r4, r5);

    // Once previous uses are out of scope the &mut can now be used in this example
    println!("{r5}");

    change(&mut s3);
    println!("{s3}");

}

// Calculate length with transfer of ownership
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// Calculate length using a String reference to s1
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Use a mutable reference (&mut String) to change a mutable variable
fn change(some_string: &mut String) {
    some_string.push_str(", and it is great!");
}


// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

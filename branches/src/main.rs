fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let not_a_bool = 3;
    // Rust will not attempt to convert non-boolean types
    // to a boolean

    // if not_a_bool {
    //     println!("Number was 3");
    // }

    if number != 0 {
        println!("number was not zero")
    }

    // Multiple branches with else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let new_number = if condition {5} else {6}; //if condition {5} else {"six"}; would cause an incompatible type error
    println!("The value of new_number is: {new_number}");



}

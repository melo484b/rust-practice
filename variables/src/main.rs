fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    {
        let x = x + 1;
        println!("The value of x is: {x} -in the inner scope.");
    }
    println!("The value of x is still: {x}");

    // Floating-point types default to f64
    let i = 2.0;
    let j: f32 = 3.0; 

    // Maths
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Floor of 2 / 3 is 0

    let remainder = 43 % 5;

    // Boolean type
    let t = true;

    let f: bool = false; // Explicit type annotation

    // Character type
    let c = 'z';
    let z: char = 'Z'; // Explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup_destructure = (500, 6.4, 1);
    let (x, y, z) = tup_destructure;
    println!("The value of y is: {y}");

    // Access tuple element directly
    let tup_x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup_x.0;
    let six_point_four = tup_x.1;
    let one = tup_x.2;

    // Array type - fixed length & elements of the same type
    let a = [1, 2, 3, 4, 5];
    let months = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

    // Array initialized with same value for each element
    let a_init = [3;5];

    // Access elements
    let first = a[0];
    let second = a[1];

    // Invalid access results in a runtime error - causes a panic instead of allowing access and continuing

}

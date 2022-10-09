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
}
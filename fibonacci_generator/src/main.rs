use std::io;

fn main() {
    println!("Welcome to the Fibonacci number generator.");

    loop {
        println!("Please enter the n-th number to be generated.");

        let mut number = String::new();

        io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("Calculating the {number}th number in the Fibonacci sequence.");
        let result = fibo_number(number);
        println!("The {number}-th number in the Fibonacci sequence is: {result}");
        break;
    }
}

fn fibo_number(x: u32) -> u32 {
    if x <= 1 {
        return x;
    }
    fibo_number(x - 1) + fibo_number(x - 2)
}

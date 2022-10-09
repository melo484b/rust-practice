fn main() {
    println!("Hello, world!");

    // snake_case is the convention for function and variable names
    print_measured_units(5, 'h');

    // Statements perform an action and do not return a value
    // Expressions evaluate to a resulting value
    let y = {
        // Statement
        let x = 3;
        // Expression
        x + 1
    };
    println!("The value of y is: {y}");

    let a = five();
    println!("The value of a is: {a}");

    let b = plus_one(8);
    println!("Eight plus one is: {b}");

}

// Parameter types must be declared - separated by commas if multiple
fn print_measured_units(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

// Functions return the last expression implicitly
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // Addinig a semicolon would change x + 1 into a statement
    // This would cause a mismatched types error since an i32
    // must be returned
}

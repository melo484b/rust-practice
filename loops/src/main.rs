fn main() {
    // Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter
        }
    };
    println!("The result is: {result}");

    // Loop labels to disambiguate between multiple loops
    let mut count_2 = 0;
    'counting_up: loop {
        println!("count = {count_2}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count_2 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count_2 += 1;
    }
    println!("End count = {count_2}");

    // Conditional loops with while
    let mut count_down = 3;
    
    while count_down != 0 {
        println!("{count_down}!");
        count_down -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through a collection
    // While loop example - unsafe method
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // Using for loop - the safe method
    let a_2 = [10, 20, 30, 40, 50];

    for element in a_2 {
        println!("the value is {element}");
    }

    // Countdown using .rev()
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

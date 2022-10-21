// Generics in function definitions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generics in struct definitions
// struct Point<T> {
//     // These fields must be the same type, T
//     x: T,
//     y: T,
// }
struct Point<T, U> {
    // These fields may be the same or different types
    x: T,
    y: U,
}

// Generics in method definitions
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generics in enum definitions
enum Option<T> {
    Some(T),
    None,
}

// Enum with multiple generic types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // Functions
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Structs
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point {x: 1.0, y: 4.0 };
    let integer_and_float = Point{ x: 5, y: 4.0 };
    let p = Point { x: 5, y: 10 };
    
    println!("p.x = {}", p.x());
}

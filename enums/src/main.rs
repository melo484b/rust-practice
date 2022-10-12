// IpAddr implemented as a struct
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("27.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1")
//     };
// }
// Enum implementation of IpAddr
enum IpAddr {
    V4(String),
    V6(String),
}
// Enums can contain a variety of types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Enums may also have methods defined with impl
impl Message {
    fn call(&self) {
        // Function body
    }
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // . . .
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T> replaces null in Rust
    // Must convert an Option<T> to a T before performing T operations
    // Everywhere that a value has a type that isn’t an Option<T>, 
    // you can safely assume that the value isn’t null.
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // When Quarter matches, Alaska becomes the binding for state
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);   // Matches the second arm Some(i) => Some(i + 1)
    let none = plus_one(None);  // Matches the first arm None => None

    // if let runs code when the value matches one pattern and ignores all other values
    // Reduces boilerplate needed but doesn't include exhaustive check like match
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }



}
// Match control flow construct
// Match must exhaust every possibility to be valid
// Catch-all arm must go last
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Oh, a lucky penny!"); // Run multiple lines of code
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => {
            print!("I can't seem to find any coins.");
            0
        }
    }
}

// If there’s a value inside, adds 1 to that value, else return None and do nothing
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

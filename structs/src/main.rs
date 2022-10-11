// Defining a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    // Instantiating a struct - order of fields is arbitrary
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Access a specific value with dot notation
    // The entire instance of user1 must be mutable
    // Rust does not allow marking of certain fields as mutable
    user1.email = String::from("anotheremail@example.com");

    // Create a struct with values from user1
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("anotheremail@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // Create a struct using struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        // .. specifies that remaining fields get values from corresponding fields in user1
        // Note: The username String data from user1 is no longer valid as it has been moved into user2
        // and does not implement the Copy trait
        ..user1
    };

    // Tuple structs - each struct is its own type, even they have the 
    // same types within
    // Similar to tuples - can be destructured into original pieces
    // .index accesses an individual value
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-like structs without fields
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}

// Returns a User instance
// Field init shorthand allows parameters to fill fields of the same name
fn build_user(email: String, username: String) -> User {
    User {
    //  email: email,
        email,
    //  username: username, 
        username,
        active: true,
        sign_in_count: 1,
    }
}

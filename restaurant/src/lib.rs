// src/main.rs and src/lib.rs are called crate roots

// mod followed by the name of the module defines a module
mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    // Making an enum public gives access to all its variants
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

}

// Re-Exporting example
// pub use crate::front_of_house::hosting;

// Rust preference is to specify absolute paths -- more likely to move code definitions 
// and item calls independently of each other.
pub fn eat_at_restaurant () {
    // eat_at_restaurant() can access add_to_waitlist() because of use..hosting being declared in scope
    hosting::add_to_waitlist();
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    // front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub trait Summary{
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Default implementation of the summarize method
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        todo!()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl trait syntax - great for simple cases
// Good if we want both item 1 and 2 to have different types
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait bound syntax
// Good if we want to force item 1 and 2 to have the same type
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds with + syntax
// pub fn notify(item: &(impl Summary + Display))
// + syntax allows trait bounds on generic types
// pub fn notify<T: Summary + Display>(item: &T)

// Clearer trait bounds with where clauses
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {...
// Becomes:
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where 
//     T: Display + Clone,
//     U: Clone + Debug,
// {}
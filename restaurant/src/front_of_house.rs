// Other modules can be placed inside
pub mod hosting;   // Making hosting public does not allow access to inner functions
mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
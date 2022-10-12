// Use nested paths to clean up large import lists (reduce vertical space taken)
// Use the as keyword to bring in types of the same name as another name
use std::{ collections::HashMap, fmt, io as IoResult };
// Combined paths
use std::io::{ self, Write };
// Glob operator * brings in all public items from the specified path
use std::collections::*;

fn main() {
    // Idiomatic use of paths
    let mut map = HashMap::new();
    map.insert(1, 2);

    // Multiple types example
    // fn function1() -> fmt::Result {
        
    // }
    
    // fn function2() -> io::IoResult<()> {
        
    // }
}
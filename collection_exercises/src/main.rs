
// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

// Convert strings to pig latin. 
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
// Keep in mind the details about UTF-8 encoding!

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use::std::{vec, collections::HashMap};

fn main() {
    median_and_mode();
}

fn median_and_mode() {
    let mut int_vec = vec![0, 4, 5, 6, 3, 3, 8];
    let mut int_map: HashMap<i32, i32> = HashMap::new();

    int_vec.sort();
    
    for num in &int_vec {
        let count = int_map.entry(*num).or_insert(0);
        *count += 1
    }

    let median = int_vec[int_vec.len()/2];

    let mut mode: i32 = 0;
    let mut most_occurances: i32 = 0;
    for (number, occurances) in int_map {
        if occurances > most_occurances {
            most_occurances = occurances;
            mode = number;
        }
    }

    print!("The median is: {}\nThe mode is: {}\n", median, mode);
    print!("{:?}", int_vec);
}

fn pig_latin() {

}

fn employee_department_interface() {

}

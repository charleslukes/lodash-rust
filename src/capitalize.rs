//!  Converts the first character of `String` to upper case and the remaining to lower case.
//! 
//! Example
//! ```
//! use lodash_rust::capitalize;
//! 
//! fn main() {
//!  let greet = String::from("hello world");
//!  let res = capitalize::new(greet);
//!  println!("{res}") // Hello world
//! }
//! ```
//!  


pub fn new(string: String) -> String {
    let mut c = string.chars();
    match c.next() {
        None => string,
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[test]
fn test_new() {
    let test_one = String::from("fred");
    assert_eq!(new(test_one), "Fred");

    let test_two = String::from("Fred");
    assert_eq!(new(test_two), "Fred");

    let test_three = String::from(" fred");
    assert_eq!(new(test_three), " fred");
}
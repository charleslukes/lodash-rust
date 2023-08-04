//! converts `value` as an array if it's not one.
//! 
//! Example
//! ```
//! use lodash_rust::cast_array;
//! 
//! fn main() {
//!  let greet = String::from("hello world");
//!  let res = cast_array::new(greet);
//!  println!("{res}") // ["hello world"]
//! }
//! ```
//! 


pub fn new<T>(data: T) -> Vec<T> {
    vec![data]
}

#[test]
fn test_new() {
    let test_one = String::from("fred");
    assert_eq!(new(test_one), ["fred"]);

    let test_two = 1;
    assert_eq!(new(test_two), [1]);
}
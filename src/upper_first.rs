//!  Converts the first character of `string` to upper case.
//!
//! Example
//! ```
//! use lodash_rust::upper_first;
//!
//! fn main() {
//!  let value = String::from("foo")
//!  let res = upper_first::new(value);
//!  println!("{res}") // "Foo"
//! }
//! ```
//!

use crate::capitalize;

pub fn new(s: &str) -> String {
    capitalize::new(s.to_owned())
}

#[test]
fn test_new() {
    let test_one = "fred";
    assert_eq!(new(test_one), "Fred");

    let test_two = "Fred";
    assert_eq!(new(test_two), "Fred");
}

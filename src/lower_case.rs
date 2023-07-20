//!  Converts `string`, as space separated words, to lower case.
//!
//! Example
//! ```
//! use lodash_rust::lower_case;
//! 
//! fn main() {
//!  let value = String::from("--foo-bar--")
//!  let res = lower_case::new(value);
//!  println!("{res}") // "foo bar"
//! }
//! ```
//!

use crate::start_case;

pub fn new(s: &str) -> String {
   start_case::new(s).to_lowercase()
}

#[test]
fn test_new() {
    assert_eq!(new("--foo-bar--"), "foo bar");
    assert_eq!(new("fooBar"), "foo bar");
    assert_eq!(new("__FOO_BAR__"), "foo bar");
}

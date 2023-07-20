//!  Converts `string`, as space separated words, to upper case.
//!
//! Example
//! ```
//! use lodash_rust::upper_case;
//! 
//! fn main() {
//!  let value = String::from("--foo-bar--")
//!  let res = upper_case::new(value);
//!  println!("{res}") // "FOO BAR"
//! }
//! ```
//!

use crate::start_case;

pub fn new(s: &str) -> String {
   start_case::new(s).to_ascii_uppercase()
}

#[test]
fn test_new() {
    assert_eq!(new("--foo-bar--"), "FOO BAR");
    assert_eq!(new("fooBar"), "FOO BAR");
    assert_eq!(new("__FOO_BAR__"), "FOO BAR");
}

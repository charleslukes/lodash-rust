//!  Converts `String` to [start case](https://en.wikipedia.org/wiki/Letter_case#Stylistic_or_specialised_usage).
//!
//! Example
//! ```
//! use lodash_rust::start_case;
//! 
//! fn main() {
//!  let value = String::from("--foo-bar--")
//!  let res = start_case::new(value);
//!  println!("{res}") // "Foo Bar"
//! }
//! ```
//!

extern crate regex;

pub fn new(s: &str) -> String {
    let re = regex::Regex::new("[^a-zA-Z0-9]+").unwrap();
    let result = re.replace_all(s, " ");
    let result = result.trim();

    let chars_list = result.chars().into_iter().collect::<Vec<char>>();
    let mut build_string = String::new();

    let mut tmp = [0u8; 4];

    for value in 0..chars_list.len() {
        let mut current_char = chars_list[value];

        // encode_utf8(&mut tmp)
        if current_char.is_ascii_lowercase() && value == 0 {
            current_char = current_char.to_ascii_uppercase();
        }

        if value > 0 && chars_list[value - 1] == ' ' {
            current_char = current_char.to_ascii_uppercase();
        }

        if value > 0 && current_char.is_uppercase() && chars_list[value - 1].is_ascii_lowercase() {
            build_string.push_str(" ");
            current_char = current_char.to_ascii_uppercase();
        }

        build_string.push_str(current_char.encode_utf8(&mut tmp));
    }

    return build_string;
}

#[test]
fn test_new() {
    assert_eq!(new("--foo-bar--"), "Foo Bar");
    assert_eq!(new("fooBar"), "Foo Bar");
    assert_eq!(new("__FOO_BAR__"), "FOO BAR");
}

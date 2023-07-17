//!  Converts `String` to [camel case](https://en.wikipedia.org/wiki/CamelCase).
//! 
//! Example
//! ```
//! use lodash_rust::camel_case;
//! 
//! fn main() {
//!  let value = String::from("enable 6h format");
//!  let res = camel_case::new(value);
//!  println!("{res}") // enable6HFormat
//! }
//! ```
//! 
extern crate regex;

use camel_case::regex::Captures;

pub fn new(string: String) -> String {
    //regex to substitute
    let re = regex::Regex::new(
			r"(?x)
			(?P<cut>[^a-zA-Z\d]+) # every char not in camelCase
			(?:
				(?P<replace>
					\d+[a-zA-Z]  # example: 24h
					|
					\d+    # example: 500
					|
					[a-zA-Z]  # example: format
				)
			|$)"
    )
    .unwrap();
    let prep = string.to_lowercase();
    let ret = re.replace_all(&prep, |caps: &Captures| {
        format!(
            "{}",
            &caps.get(2).map(|n| n.as_str()).unwrap_or("").to_uppercase()
        )
    });
    if ret.chars().next().map(|c| !c.is_ascii()).unwrap_or(true) {
        return ret.to_string();	//error handling for when String size is less than two
    }
    let tail = &ret[1..];

    format!("{}{tail}", ret.chars().next().unwrap().to_lowercase())
}

#[test]
fn test_new() {
    let test_one = String::from("12 feet");
    assert_eq!(new(test_one), "12Feet");

    let test_two = String::from("enable 6h format");
    assert_eq!(new(test_two), "enable6HFormat");

    let test_three = String::from("enable 24H format");
    assert_eq!(new(test_three), "enable24HFormat");

    let test_four = String::from("too legit 2 quit");
    assert_eq!(new(test_four), "tooLegit2Quit");

    let test_five = String::from("walk 500 miles");
    assert_eq!(new(test_five), "walk500Miles");

    let test_six = String::from("xhr2 request");
    assert_eq!(new(test_six), "xhr2Request");

    let test_seven = String::from("--xhr--request--");
    assert_eq!(new(test_seven), "xhrRequest");

    let test_eight = String::from("__FOO_BAR__");
    assert_eq!(new(test_eight), "fooBar");

    let test_nine = String::from("foo 2000_ha");
    assert_eq!(new(test_nine),"foo2000Ha");
}

/// Converts the first character of `string` to upper case and the remaining to lower case.
///
/// @category String
/// @param {String} [string=''] The string to capitalize.
/// @returns {String} Returns the capitalized string.
/// 


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
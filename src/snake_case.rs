extern crate regex;

///  Converts `String` to  [snake case](https://en.wikipedia.org/wiki/Snake_case).
///
/// @category String
/// @param {String} [string=''] The string to convert.
/// @returns {String} Returns the kebab cased string.
///

pub fn new(s: &str) -> String {
    let re = regex::Regex::new("[^a-zA-Z0-9]+").unwrap();
    let result = re.replace_all(s, " ");
    let result = result.trim();
    let result_string = re.replace_all(result, "_");

    // check if string contains "-"
    let re2 = regex::Regex::new("[_]+").unwrap();
    let contains_underscore = re2.is_match(result_string.as_ref());

    // check if contains numbers
    let re3 = regex::Regex::new("[0-9]+").unwrap();
    let contains_number = re3.is_match(result_string.as_ref());

    let mut build_result_string = String::new();

    if !contains_underscore | contains_number {
        let mut tmp = [0u8; 4];
        // convert to char
        let characters: Vec<char> = result_string.chars().collect();
        for letter in characters {
            if letter.is_uppercase() {
                build_result_string.push_str("_");
                build_result_string.push_str(letter.encode_utf8(&mut tmp));
            } else if letter.is_numeric() {
                build_result_string.push_str("_");
                build_result_string.push_str(letter.encode_utf8(&mut tmp));
                build_result_string.push_str("_");
            } else {
                build_result_string.push_str(letter.encode_utf8(&mut tmp));
            }
        }

        return build_result_string.to_lowercase();
    }

    return result_string.to_string().to_lowercase();
}

#[test]
fn test_new() {
    assert_eq!(new("Foo Bar"), "foo_bar");
    assert_eq!(new("fooBar"), "foo_bar");
    assert_eq!(new("__FOO_BAR__"), "foo_bar");
    assert_eq!(new("foo2bar"), "foo_2_bar");
}

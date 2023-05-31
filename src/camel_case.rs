///  Converts `string` to [camel case](https://en.wikipedia.org/wiki/CamelCase).
/// 
///  @param {String} [string=''] The string to convert.
///  @returns {String} Returns the camel cased string.
///  @see lower_case, kebab_case, snake_case, start_case, upper_case, upper_first
/// 


// TODO: Rewrite just using regex
pub fn new(string: String) -> String {
    let mut combine_result: String = String::new();
    let str_lower = string.to_ascii_lowercase();
    let str_trim = str_lower.trim();
    let mut str_array: Vec<char> = str_trim.chars().collect();

    for i in 0..str_array.len() {
        let char_to_string = str_array[i].to_string();
        if char_to_string.trim().is_empty() {
            // check the next character if number find next letter char
            let mut index = i + 1;
            while index <= str_array.len() {
                if str_array[index].is_numeric() {
                    index = index + 1;
                    continue;
                }
                str_array[index] = str_array[index].to_ascii_uppercase();
                break;
            }
            continue;
        }
        combine_result.push_str(&char_to_string)
    }

    return combine_result;
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
}
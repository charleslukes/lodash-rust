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

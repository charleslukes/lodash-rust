pub fn new(string: String) -> String {
    let string_trim = string.trim().to_owned();
    let mut c = string_trim.chars();
    match c.next() {
        None => string,
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

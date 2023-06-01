pub fn from_str(s: &str) -> String {
    return s.chars().flat_map(|c| c.to_lowercase()).collect();
}

pub fn new(s: String) -> String {
    return s.chars().flat_map(|c| c.to_lowercase()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(from_str("HELLO, WORLD!"), "hello, world!");
        assert_eq!(from_str("Hello, World!"), "hello, world!");
        assert_eq!(from_str("hello, world!"), "hello, world!");
        assert_eq!(from_str(""), "");
    }

    #[test]
    fn test_new() {
        assert_eq!(new("HELLO, WORLD!".to_string()), "hello, world!");
        assert_eq!(new("Hello, World!".to_string()), "hello, world!");
        assert_eq!(new("hello, world!".to_string()), "hello, world!");
        assert_eq!(new("".to_string()), "");
    }
}

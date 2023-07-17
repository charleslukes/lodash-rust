//! Creates a slice of `array` excluding elements dropped from the end.
//! Elements are dropped until `predicate` returns falsy. 
//! 
//! Example
//! ```
//! use lodash_rust::drop_right_while;
//! 
//! fn main() {
//!  let res = drop_right_while::new(new([1, 2, 3, 4].to_vec(), &|x: i32| x > 2));
//!  println!("{res}") // [1, 2]
//! }
//! ```
//! 

pub fn new<T: Copy>(array: Vec<T>, f: &dyn Fn(T) -> bool) -> Vec<T> {
    let mut result = Vec::new();
    let array_len = array.len();

    for index in 0..array_len {
        // from the end
        let current_value = array[(array_len - 1) - index];
        if !f(current_value) {
            let mut value = [current_value].to_vec();
            value.append(&mut result);
            result = value;
        }
    }

    return result;
}

#[test]
fn test_new() {
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x > 2), [1, 2]);
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 5), []);
}

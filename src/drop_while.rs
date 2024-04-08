//! Creates a slice of `array` excluding elements dropped from the beginning.
//! Elements are dropped until `predicate` returns falsy.
//!
//! Example
//! ```
//! use lodash_rust::drop_while;
//!
//! let res = drop_while::new([1, 2, 3, 4].to_vec(), &|x: i32| x < 3);
//! println!("{res:?}") // [3, 4]
//! ```

pub fn new<T: Copy>(array: Vec<T>, f: &dyn Fn(T) -> bool) -> Vec<T> {
    let mut result = Vec::new();

    for current_value in array {
        if !f(current_value) {
            result.push(current_value);
        }
    }

    result
}

#[test]
fn test_new() {
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 3), [3, 4]);
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 5), []);
}

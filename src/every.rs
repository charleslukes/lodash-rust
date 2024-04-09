//! Checks if `predicate` returns truthy for **all** elements of `array`
//! Iteration is stopped once `predicate` returns falsy. The predicate is
//! __Note:__ This method returns `true` for
//! [empty vector](https://en.wikipedia.org/wiki/Empty_set) because
//! [everything is true](https://en.wikipedia.org/wiki/Vacuous_truth) of
//! elements of empty vector.
//!
//! Example
//! ```
//! use lodash_rust::every;
//!
//! let res = every::new([true, false].to_vec(), &|x: bool| x);
//! println!("{res}") // false
//! ```

pub fn new<T: Copy>(array: Vec<T>, f: &dyn Fn(T) -> bool) -> bool {
    let mut result = true;

    for current_value in array {
        if !f(current_value) {
            result = false;
            break;
        }
    }

    result
}

#[test]
fn test_new() {
    assert!(!new([1, 2, 3, 4].to_vec(), &|x: i32| x < 3));
    assert!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 5));
    assert!(!new([true, false].to_vec(), &|x: bool| x));
    assert!(!new([false, false].to_vec(), &|x: bool| x));
    assert!(new([true, true].to_vec(), &|x: bool| x));
    assert!(new([].to_vec(), &|x: bool| x));
}

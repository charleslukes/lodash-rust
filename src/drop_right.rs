//! Creates a slice of `array` with `n` elements dropped from the end.
//! 
//! Example
//! ```
//! use lodash_rust::drop_right;
//! 
//! fn main() {
//!  let res = drop_right::new([1, 2, 3].to_vec(), Some(2));
//!  println!("{res}") // [1]
//! }
//! ```
//! 

use std::convert::TryInto;

pub fn new<T: PartialOrd + Eq + Clone>(array: Vec<T>, number: Option<i32>) -> Vec<T> {
    // defaults to 1
    let num = number.unwrap_or(1);
    let arr_len = array.len().try_into().unwrap();
    if num > arr_len {
        return [].to_vec();
    } else {
        let slice_len = arr_len - num;
        return array[..slice_len as usize].to_vec();
    }
}

#[test]
fn test_new() {
    assert_eq!(new([1, 2, 3].to_vec(), None), [1, 2]);
    assert_eq!(new([1, 2, 3].to_vec(), Some(2)), [1]);
    assert_eq!(new([1, 2, 3].to_vec(), Some(5)), []);
    assert_eq!(new([1, 2, 3].to_vec(), Some(0)), [1, 2, 3]);
}

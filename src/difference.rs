//! Creates an array of `array` values not included in the other given arrays
//! using [`SameValueZero`](http://ecma-international.org/ecma-262/7.0/#sec-samevaluezero)
//! 
//! for equality comparisons. The order and references of result values are
//! determined by the first array.
//! 
//! Example
//! ```
//! use lodash_rust::difference;
//! 
//! fn main() {
//!  let original_vector = [2, 1, 2, 3].to_vec();
//!  let vector_to_check = [[3, 2].to_vec(), [3, 4].to_vec()].to_vec();
//!  let res = difference::new(original_vector, vector_to_check);
//!  println!("{res}") // [1]
//! }
//! ```
//!  

use std::collections::HashSet;
use std::hash::Hash;

pub fn new<T: PartialOrd + Eq + Hash + Clone>(array: Vec<T>, values: Vec<Vec<T>>) -> Vec<T> {
    let mut new_array = array.clone();

    let only_unique_values = &values
        .concat()
        .into_iter()
        .map(|a| a)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<T>>();

    for unique_value in only_unique_values {
        if array.contains(unique_value) {
            new_array = new_array
                .iter()
                .filter(|a| a != &unique_value)
                .cloned()
                .collect();
        }
    }

    new_array
}

#[test]
fn test_new() {
    let original_vector = [2, 1, 2, 3].to_vec();
    let vector_to_check = [[3, 2].to_vec(), [3, 4].to_vec()].to_vec();
    assert_eq!(new(original_vector, vector_to_check), [1]);
}

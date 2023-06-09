use std::convert::TryInto;
use std::hash::Hash;

/// Creates a slice of `array` with `n` elements dropped from the beginning.
///
/// @category Vec
/// @param {Vec} array The vector to query.
/// @param {number} [n=1] The number of elements to drop.
/// @returns {Vec} Returns the slice of `Vec`.
///

pub fn new<T: PartialOrd + Ord + Hash + Eq + Clone>(array: Vec<T>, number: Option<i32>) -> Vec<T> {
    let num = number.unwrap_or(1);
    let arr_len = array.len().try_into().unwrap();
    if num > arr_len {
        return [].to_vec();
    } else {
        return array[num as usize..].to_vec();
    }
}

#[test]
fn test_new() {
    assert_eq!(new([1, 2, 3].to_vec(), None), [2, 3]);
    assert_eq!(new([1, 2, 3].to_vec(), Some(2)), [3]);
    assert_eq!(new([1, 2, 3].to_vec(), Some(5)), []);
    assert_eq!(new([1, 2, 3].to_vec(), Some(0)), [1, 2, 3]);
}

use std::fmt::Display;

///
/// This method is like `find` except that it iterates over elements of
/// `collection` from right to left.
///
/// @category Collection
/// @param {Vector} collection The collection to inspect.
/// @param {Function} predicate The function invoked per iteration.
/// @param {number} [fromIndex=collection.length-1] The index to search from.
/// @returns {*} Returns the matched element, else `None`.
///

pub fn new<T: Copy + Display>(
    array: Vec<T>,
    f: &dyn Fn(T) -> bool,
    from_index: usize,
) -> Option<T> {
    if from_index > array.len() - 1 {
        return None;
    }

    let mut loop_index = 1;

    if from_index > 0 {
        loop_index = from_index;
    }

    for index in 0..loop_index {
        let value = array[from_index - index];
        if f(value) {
            return Some(value);
        }
    }

    return None;
}

#[test]
fn test_new() {
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 3, 3), Some(2));
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x % 2 == 1, 3), Some(3));
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 3, 5), None);
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x == 1, 0), Some(1));
}
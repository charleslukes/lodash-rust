///
/// Creates a slice of `array` excluding elements dropped from the beginning.
/// Elements are dropped until `predicate` returns falsy. The predicate is
/// invoked with three arguments: (value, index, array).
///
/// @category Vec
/// @param {Vec} array The vector to query.
/// @param {Function} predicate The function invoked per iteration.
/// @returns {Vec} Returns the slice of `Vec`.
///

pub fn new<T: Copy>(array: Vec<T>, f: &dyn Fn(T) -> bool) -> Vec<T> {
    let mut result = Vec::new();

    for index in 0..array.len() {
        let current_value = array[index];
        if !f(current_value) {
            result.push(current_value);
        }
    }

    return result;
}

#[test]
fn test_new() {
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 3), [3, 4]);
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 5), []);
}

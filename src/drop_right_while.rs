///
/// Creates a slice of `array` excluding elements dropped from the end.
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

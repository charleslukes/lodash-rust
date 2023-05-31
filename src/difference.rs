use std::collections::HashSet;

/// Creates an array of `array` values not included in the other given arrays
/// using [`SameValueZero`](http://ecma-international.org/ecma-262/7.0/#sec-samevaluezero)
/// for equality comparisons. The order and references of result values are
/// determined by the first array.
///
/// @category Vec
/// @param {Vec}  The vector to inspect.
/// @param {Vec<Vec>} [values] The values to exclude.
/// @returns {Vec} Returns the new vector of filtered values.

// TODO: make i32 a generic 
pub fn new(array: Vec<i32>, values: Vec<Vec<i32>>) -> Vec<i32> {
    let mut new_array = array.clone();
    
    let only_unique_values = &values.concat()
        .into_iter()
        .map(|a| a)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<i32>>();

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
    let original_vector =[2, 1, 2, 3].to_vec();
    let vector_to_check = [[3, 2].to_vec(), [3, 4].to_vec()].to_vec();
    assert_eq!(new(original_vector, vector_to_check), [1]);

   
}

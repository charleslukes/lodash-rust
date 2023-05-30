use std::collections::HashSet;

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

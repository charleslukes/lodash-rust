//! Creates an array of elements split into groups the length of `size`.
//! If `array` can't be split evenly, the final chunk will be the remaining
//! elements.
//!
//! Example
//! ```
//! use lodash_rust::chunk;
//!
//! fn main() {
//!  let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
//!  let res = chunk::new(numbers);
//!  println!("{:?}", res) // [[1, 2, 3], [4, 5, 6]]
//! }
//! ```
//!  
use std::fmt::Debug;

pub fn new<T: Clone + Debug>(arr: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let arr_len = arr.len();
    let div_num = arr_len / size;
    let remainder = arr_len % size;
    let mut result: Vec<Vec<T>> = Vec::new();
    let mut index = 0;
    let mut counter = 0;

    if div_num > 0 {
        while counter < (div_num * size) {
            index += 1;
            let end = size * index;
            result.push(arr[counter..end].to_vec());
            counter += size;
        }
    }
    if remainder > 0 {
        result.push(arr[counter..].to_vec());
    }
    result
}

#[test]
fn test_new() {
    let num: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(new(num, 3), vec![vec![1, 2, 3], vec![4, 5, 6]]);

    let num: Vec<i32> = vec![1, 2, 3];
    assert_eq!(new(num, 2), vec![vec![1, 2], vec![3]]);
}

//! This method is like `find` except that it returns the key of the first
//! element `predicate` returns truthy for instead of the element itself.
//!
//! Example
//! ```
//! use lodash_rust::find_key;
//!
//! fn main() {
//!    let mut map: HashMap<char, u64> = HashMap::new();
//!     map.insert('a', 1);
//!     map.insert('b', 2);
//!     map.insert('c', 3);
//!
//!    let res = find_key::new(&map, |&x| x % 2 == 1);
//!    println!("{res}") // Some('c')
//! }
//! ```

use std::collections::HashMap;

pub fn new<K: Copy, V, F: Fn(&V) -> bool>(h: &HashMap<K, V>, p: F) -> Option<K> {
    for (k, v) in h {
        if p(v) {
            return Some(*k);
        }
    }
    None
}

#[test]
fn test_new() {
    let mut map: HashMap<char, u64> = HashMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    assert_eq!(new(&map, |&x| x % 2 == 1), Some('c'));
    assert_eq!(new(&map, |&x| x == 2), Some('b'));
}

//! Checks if `predicate` returns truthy for **all** properties of `HashMap`.
//! Iteration is stopped once `predicate` returns falsy. The predicate is
//!
//! **Note:** This method returns `true` for
//! [empty HashMaps](https://en.wikipedia.org/wiki/Empty_set) because
//! [everything is true](https://en.wikipedia.org/wiki/Vacuous_truth) of
//! elements of empty HashMap.
//!
//! Example
//! ```
//! use lodash_rust::every_value;
//!
//! fn main() {
//!    let mut map: HashMap<char, u64> = HashMap::new();
//!     map.insert('a', 1);
//!     map.insert('b', 2);
//!     map.insert('c', 3);
//!
//!    let res = every_value::new(&map, |&x| x % 2 == 1);
//!    println!("{res}") // false
//! }
//! ```

use std::collections::HashMap;

pub fn new<K, V, F: Fn(&V) -> bool>(h: &HashMap<K, V>, p: F) -> bool {
    for (_k, v) in h {
        if !p(v) {
            return false;
        }
    }
    true
}

#[test]
fn test_new() {
    let mut map: HashMap<char, u64> = HashMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    assert_eq!(new(&map, |&x| x % 2 == 1), false);
    assert_eq!(new(&map, |&x| x >= 1), true);
}

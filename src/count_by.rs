//! Returns a hashmap where the key is the result of running each element through iteratee, and the
//! value is how many times the element was returned by iteratee.
//!
//! Example
//! ```
//! use lodash_rust::count_by;
//! let users = [
//!     ("barney", true),
//!     ("betty", true),
//!     ("fred", false)
//! ];
//! let res = count_by::new(&users, |user: &(&str, bool)| (*user).1);
//! println!("{res:?}") // { true: 2, false: 1 }
//! ```

use std::{collections::HashMap, hash::Hash};

pub fn new<T, F, K>(collection: &[T], iteratee: F) -> HashMap<K, usize>
where
    F: Fn(&T) -> K,
    K: Hash + Eq,
{
    let mut res = HashMap::new();
    for item in collection {
        let key = iteratee(item);
        let count = res.entry(key).or_insert(0);
        *count += 1;
    }
    res
}

#[test]
fn test_new() {
    let users = [
        ("barney", true),
        ("betty", true),
        ("fred", false),
    ];
    let res = new(&users, |user| user.1);
    assert_eq!(res.get(&true), Some(&2));
    assert_eq!(res.get(&false), Some(&1));
}

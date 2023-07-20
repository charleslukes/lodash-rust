//! This method is like `findIndex` except that it iterates over elements
//! of `collection` from right to left.
//! 
//! Returns the index of the found element, else None.
//!  
//! Example
//! ```
//! use lodash_rust::find_last_index;
//!
//! fn main() {
//!  pub struct Person {
//!     pub name: String,
//!    pub age: u32,
//! }
//!
//! impl Person {
//!    pub fn new(age: u32, name: String) -> Self {
//!     Self { name, age }
//!    }
//! }
//!
//! let person_1 = Person::new(23, String::from("John"));
//! let person_2 = Person::new(19, String::from("Cynthia"));
//! let person_3 = Person::new(20, String::from("Luke"));
//!
//! fn is_older_than_20(p: &Person) -> bool {
//!    p.age >= 20
//! }
//! let all_persons = vec![person_1, person_2, person_3];
//! let res = find_last_index::new(&all_persons, is_older_than_20)
//! println!("{res}") // Some(2)
//! }
//! ```

pub fn new<T, F: Fn(&T) -> bool>(array: &Vec<T>, p: F) -> Option<usize> {
    let index_len = array.len() - 1;
    for (i, _v) in array.iter().enumerate() {
        if p(&array[index_len - i]) {
            return Some(index_len - i);
        }
    }

    None
}

#[test]
fn test_new() {
    pub struct Person {
        pub name: String,
        pub age: u32,
    }

    impl Person {
        pub fn new(age: u32, name: String) -> Self {
            Self { name, age }
        }
    }

    let person_1 = Person::new(23, String::from("John"));
    let person_2 = Person::new(19, String::from("Cynthia"));
    let person_3 = Person::new(20, String::from("Luke"));

    fn is_older_than_20(p: &Person) -> bool {
        p.age >= 20
    }

    let all_persons = vec![person_1, person_2, person_3];

    assert_eq!(new(&all_persons, is_older_than_20), Some(2));
}

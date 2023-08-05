//! A library for javascript lodash methods in rustlang
//!
//! This library methods is meant to give the same output as 
//! their equivalent in javascript, if you notice any bugs or differences
//! please on an issue. 
//!
//! # Usage
//!
//! First, add this to your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! lodash_rust = "0.1.0"
//! ```
//!
//! Next:
//!
//! ```
//! use lodash_rust::capitalize;
//! 
//! fn main() {
//!  let greet = String::from("hello world");
//!  let res = capitalize::new(greet);
//!  println!("{res}") // Hello world
//! }
//! ```
//!

pub mod camel_case;
pub mod capitalize;
pub mod difference;
pub mod kebab_case;
pub mod snake_case;
pub mod start_case;
pub mod drop;
pub mod drop_right;
pub mod drop_while;
pub mod drop_right_while;
pub mod every;
pub mod find_last;
pub mod filter;
pub mod every_value;
pub mod find_key;
pub mod find_last_index;
pub mod lower_case;
pub mod upper_case;
pub mod upper_first;
pub mod after;
pub mod cast_array;
pub mod chunk;
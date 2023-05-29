extern crate lodash_rust;
use lodash_rust::camel_case;

fn main() {
    let result = camel_case::new("xhr2 request".to_string());
    println!("result ==> {}", result)
}

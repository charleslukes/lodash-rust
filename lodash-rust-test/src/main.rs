extern crate lodash_rust;
use lodash_rust::camel_case;
use lodash_rust::capitalize;


fn main() {
    let result = camel_case::new("xhr2 request".to_string());
    println!("result ==> {}", result);

    let capitalize = capitalize::new("fred".to_string());
    println!("result ==> {}", capitalize);

}

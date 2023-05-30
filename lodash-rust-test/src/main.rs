extern crate lodash_rust;
use lodash_rust::camel_case;
use lodash_rust::capitalize;
use lodash_rust::difference;

fn main() {
    let result = camel_case::new("xhr2 request".to_string());
    println!("result ==> {}", result);

    let capitalize = capitalize::new("fred".to_string());
    println!("result ==> {}", capitalize);

    let difference = difference::new([2, 1, 2, 3].to_vec(), [[3, 2].to_vec(), [3, 4].to_vec()].to_vec());
    println!("result ==> {:?}", difference);
}

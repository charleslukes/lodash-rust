///  
///  Iterates over elements of `array`, returning an array of all elements
///  `predicate` returns truthy for. The predicate is invoked with three
///  arguments: (value, index, array).
///  


pub fn new<T: Copy>(array: Vec<T>, f: &dyn Fn(T) -> bool) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();

    for index in 0..array.len() {
        let value = array[index];
        if f(value) {
            result.push(value);
        }
    }

    return result;
}


#[test]
fn test_new() {
    assert_eq!(new([1, 2, 3, 4].to_vec(), &|x: i32| x < 3), [1, 2]);
    assert_eq!(new([false, false, false, false].to_vec(), &|x: bool| x), []);
    assert_eq!(new([false, true, false, false].to_vec(), &|x: bool| !x), [false, false, false]);
}

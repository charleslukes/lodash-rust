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

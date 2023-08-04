//!  The opposite of `before`. This method creates a function that invokes
//! `func` once it's called `n` or more times.
//!
//! Example
//! ```
//! use lodash_rust::after{After};
//!
//! fn main() {
//!  let mut after = After::new();
//!  // doesn't call now
//!  after.call(2, || println!("Hello World..."));
//!  // gets called now and print "Hello World..."
//!  after.call(2, || println!("Hello World..."));
//! }
//! ```
//!  

pub struct After {
    count: usize,
}

impl After {
    pub fn new() -> Self {
        After { count: 0 }
    }
    pub fn call<F>(&mut self, n: usize, func: F)
    where
        F: Fn() -> (),
    {
        self.count += 1;

        if self.count == n {
            func();
            // reinitialize counter back to 0
            self.count = 0;
        }
    }
}

#[test]
fn test_new() {
    let mut after = After::new();
    // doesn't call now
    after.call(3, || println!("Hello World..."));
    // should run once but not call func
    assert_eq!(after.count, 1);

    after.call(3, || println!("Hello World..."));
    // should run twice but not call func
    assert_eq!(after.count, 2);

    after.call(3, || println!("Hello World..."));
    // should call func and reset count to zero
    assert_eq!(after.count, 0);
}

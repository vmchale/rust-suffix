#![feature(test)]

pub use functions::*;

pub mod test;

/// Separate module for functions, so that they can be called in the test module.
pub mod functions {

    use std::mem::replace;

    /// this function returns a vector of all possible suffixes, e.g.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_suffix::functions::*;
    ///
    /// let v: Vec<String> = vec!["ails".to_string(),"ils".to_string(),"ls".to_string(),"s".to_string()];
    /// let s = "tails".to_string();
    /// assert_eq!(v, suffix_vec(s))
    /// ```
    pub fn suffix_vec(s: String) -> Vec<String> {
        let mut vec = Vec::new();
        for i in 1..s.len() {
            let next = s.chars().skip(i).take(i-s.len()).collect();
            vec.push(next);
        }
        vec
    }

    pub fn try() -> () {
        let v: Vec<String> = vec!["ails".to_string(),"ils".to_string(),"ls".to_string(),"s".to_string()];
        let s = "tails".to_string();
        assert_eq!(v, suffix_vec(s))
    }

    /// # Examples
    ///
    /// ```
    /// use rust_suffix::functions::*;
    ///
    /// assert_eq!(89, fib_recursive(10))
    /// ```
    pub fn fib_recursive(n: i32) -> i32 {
        match n {
            0 | 1 => 1,
            _ => fib_recursive(n - 1) + fib_recursive(n - 2)
        }
    }

    #[derive(Default)]
    pub struct Fib {
        curr: i32,
        next: i32,
    }

    impl Iterator for Fib {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            let new_next = self.curr + self.next;
            let new_curr = replace(&mut self.next, new_next);
            Some(replace(&mut self.curr, new_curr))
        }
    }

    impl Fib {
        pub fn new() -> Fib {
            Fib { curr: 1, next: 1 }
        }
    }

    /// # Examples
    ///
    /// ```
    /// use rust_suffix::functions::*;
    ///
    /// assert_eq!(89, fib_iterative(10))
    pub fn fib_iterative(n: usize) -> i32 {
        let fib = Fib::new();
        let v: Vec<i32> = fib.take(n+1).collect();
        v[n]
    }
}

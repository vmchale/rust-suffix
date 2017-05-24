#![feature(test)]

pub use functions::*;

pub mod test;

/// Separate module for functions, so that they can be called in the test module.
pub mod functions {
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
}

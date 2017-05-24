//! Module containing benchmarks
#![allow(unused_imports)]
extern crate test;

use test::test::Bencher;
use functions::*;

#[bench]
fn bench_suffix(b: &mut Bencher) {
    let s = "tails".to_string();
    b.iter(|| suffix_vec(s.clone()));
}

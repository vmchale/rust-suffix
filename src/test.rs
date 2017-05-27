//! Module containing benchmarks
#![allow(unused_imports)]
//#[overflow_checks(off)]
extern crate test;

use std::mem::replace;
use test::test::Bencher;
use functions::*;

#[bench]
fn bench_suffix(b: &mut Bencher) {
    let s = "tails".to_string();
    b.iter(|| suffix_vec(s.clone()));
}

#[bench]
fn bench_recursive_fib(b: &mut Bencher) {
    b.iter(|| { (0..15).map(fib_recursive).collect::<Vec<i32>>()[14] } )
}

#[bench]
fn bench_iterative_fib(b: &mut Bencher) {
    b.iter(|| { fib_iterative(15) })
}

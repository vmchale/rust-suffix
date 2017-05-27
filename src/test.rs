//! Module containing benchmarks

#![allow(unused_imports)]
extern crate test;

use std::mem::replace;
use test::test::Bencher;
use functions::*;

// mostly here to be compared to Haskell/GHC.
#[bench]
fn bench_suffix(b: &mut Bencher) {
    let s = "tails".to_string();
    b.iter(|| suffix_vec(s.clone()));
}

// unfortunately, unless we use a map, it won't bother benchmark correctly
#[bench]
fn bench_recursive_fib(b: &mut Bencher) {
    b.iter(|| { (1..15).map(fib_recursive).collect::<Vec<i32>>()[13] } )
}

// this one doesn't need any special considerations
#[bench]
fn bench_iterative_fib(b: &mut Bencher) {
    b.iter(|| { fib_iterative(15) })
}

#[bench]
fn bench_big_fib(b: &mut Bencher) {
    b.iter(|| { fib_big(15) })
}

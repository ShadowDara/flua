#![feature(test)]

extern crate test;

use test::Bencher;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[bench]
fn bench_fibonacci_2(b: &mut Bencher) {
    b.iter(|| fibonacci(2));
}

#[bench]
fn bench_fibonacci_20(b: &mut Bencher) {
    b.iter(|| fibonacci(20));
}

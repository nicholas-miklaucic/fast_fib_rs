use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use fast_fib::{
    Binet, BinetZ5, Cassini, CassiniGMP, DPIterator, FibFinder, Integer, MatExponentiator,
    MemoizedRecursor, NaiveRecursor, GMP,
};

/// Functions applicable for small numbers: e.g., every algorithm.
fn small_fns() -> Vec<(Box<dyn Fn(u64) -> Integer>, &'static str)> {
    vec![
        (
            Box::new(|x| NaiveRecursor::default().fib(x)),
            "Naïve Recursive",
        ),
        (
            Box::new(|x| MemoizedRecursor::default().fib(x)),
            "Memoized Recursive",
        ),
        (Box::new(|x| DPIterator::default().fib(x)), "DP"),
        (
            Box::new(|x| MatExponentiator::default().fib(x)),
            "Matrix Exponentiation",
        ),
        (Box::new(|x| Binet::default().fib(x)), "Binet"),
        (Box::new(|x| BinetZ5::default().fib(x)), "Binet (Z5)"),
        (
            Box::new(|x| Cassini::default().fib(x)),
            "Efficient Matrix Exponentiation",
        ),
        (
            Box::new(|x| CassiniGMP::default().fib(x)),
            "GMP Algorithm Port",
        ),
        (Box::new(|x| GMP::default().fib(x)), "GMP"),
    ]
}

/// Functions applicable for medium numbers: this excludes the naïve approach.
fn medium_fns() -> Vec<(Box<dyn Fn(u64) -> Integer>, &'static str)> {
    small_fns().into_iter().skip(1).collect()
}

/// Functions applicable for large numbers: this excludes all of the exponential-time algorithms.
fn large_fns() -> Vec<(Box<dyn Fn(u64) -> Integer>, &'static str)> {
    small_fns().into_iter().skip(3).collect()
}

/// Functions applicable for the largest numbers: this is only Cassini-based approaches and the Binet-Z5 approach.
fn largest_fns() -> Vec<(Box<dyn Fn(u64) -> Integer>, &'static str)> {
    small_fns().into_iter().skip(5).collect()
}

pub fn fib_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci (small)");
    group
        .nresamples(5000)
        .measurement_time(Duration::new(2, 0))
        .warm_up_time(Duration::new(1, 0));
    let nums = [2, 3, 5, 10, 15, 25];
    for i in nums {
        for (f, name) in small_fns() {
            group.bench_with_input(BenchmarkId::new(name, i), &i, |b, i| b.iter(|| f(*i)));
        }
    }
    group.finish();
}

pub fn fib_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci (medium)");
    group
        .measurement_time(Duration::new(2, 0))
        .warm_up_time(Duration::new(1, 0));
    // much larger than this, and you stack overflow for memoized
    let nums = [50, 200, 800, 1600, 6400, 25600, 52800];
    for i in nums {
        for (f, name) in medium_fns() {
            group.bench_with_input(BenchmarkId::new(name, i), &i, |b, i| b.iter(|| f(*i)));
        }
    }
    group.finish();
}

pub fn fib_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci (large)");
    group
        .sample_size(40)
        .measurement_time(Duration::new(2, 0))
        .warm_up_time(Duration::new(1, 0));
    let nums = [100_000, 400_000, 1_600_000, 6_400_000, 25_600_000];
    for i in nums {
        for (f, name) in large_fns() {
            group.bench_with_input(BenchmarkId::new(name, i), &i, |b, i| b.iter(|| f(*i)));
        }
    }
    group.finish();
}

pub fn fib_largest(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci (largest)");
    group
        .sample_size(10)
        .measurement_time(Duration::new(2, 0))
        .warm_up_time(Duration::new(1, 0));
    let nums = [100_000_000, 500_000_000, 1_000_000_000, 4_000_000_000];
    for i in nums {
        for (f, name) in largest_fns() {
            group.bench_with_input(BenchmarkId::new(name, i), &i, |b, i| b.iter(|| f(*i)));
        }
    }
    group.finish();
}

criterion_group!(small, fib_small);
criterion_group!(medium, fib_medium);
criterion_group!(large, fib_large);
criterion_group!(largest, fib_largest);
criterion_main!(small, medium, large, largest);

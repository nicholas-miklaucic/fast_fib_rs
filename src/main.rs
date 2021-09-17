use crate::fib_finder::FibFinder;

mod binet;
mod dp_iterator;
mod fib_finder;
mod mat_exponentiator;
mod memoized;
mod naive;
mod repeated_squaring;

fn main() {
    let mut alg = mat_exponentiator::MatExponentiator::default();
    println!(
        "F(10^9) = ...{}",
        alg.fib(2 * 10_u64.pow(9_u32)) % 10_000_000_000_i64
    );
}

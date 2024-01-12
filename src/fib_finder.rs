//! Trait that abstracts implementations of algorithms to find the nth Fibonacci number.

use rug::Integer;

/// Represents an algorithm for finding the nth Fibonacci number.
pub trait FibFinder {
    /// Finds the nth Fibonacci number. We define it such that the 0th Fibonacci number is 0.
    fn fib(&mut self, n: u64) -> Integer;
}

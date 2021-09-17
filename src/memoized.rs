//! Recursive Fibonacci implementation that uses memoization to cause an exponential speedup.

use rug::Integer;
use std::collections::HashMap;

use crate::fib_finder::FibFinder;

/// Recursive approach, using the basic definition of Fibonacci numbers: F(0) =
/// 0, F(1) = 1, and F(n) = F(n - 1) + F(n - 2). The difference from the naïve
/// approach is that we *memoize* results, caching them to avoid duplicating
/// work.
#[derive(Clone, Debug)]
pub struct MemoizedRecursor {
    results: HashMap<u64, rug::Integer>,
}

impl Default for MemoizedRecursor {
    fn default() -> Self {
        Self {
            results: HashMap::new(),
        }
    }
}

impl MemoizedRecursor {
    /// Clears the cache, ensuring that no work is saved from previous runs.
    pub fn clear(&mut self) {
        self.results = HashMap::new();
    }
}

impl FibFinder for MemoizedRecursor {
    fn fib(&mut self, n: u64) -> Integer {
        if let Some(out) = self.results.get(&n) {
            out.clone()
        } else {
            let result = match n {
                0 => Integer::from(0),
                1 => Integer::from(1),
                _ => self.fib(n - 1) + self.fib(n - 2),
            };

            // Add to cache.
            self.results.insert(n, result.clone());
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let mut alg = MemoizedRecursor::default();
        assert_eq!(alg.fib(0), 0);
        alg.clear();
        assert_eq!(alg.fib(1), 1);
        alg.clear();
        assert_eq!(alg.fib(2), 1);
        alg.clear();
        assert_eq!(alg.fib(12), 144);
        alg.clear();
        assert_eq!(alg.fib(37), 24157817);

        // This will take a while...

        alg.clear();
        assert_eq!(
            alg.fib(100),
            "354224848179261915075".parse::<Integer>().unwrap()
        );

        alg.clear();
        assert_eq!(
            alg.fib(1000),
            ("434665576869374564356885276750406258025646605173717804024817290895365554".to_owned()
                + "1794905189040387984007925516929592259308032263477520968962323987332247116164299"
                + "6440906533187938298969649928516003704476137795166849228875")
                .parse::<Integer>()
                .unwrap()
        );

        // Uh-oh, stack overflow!
        // alg.clear();
        // assert_eq!(alg.fib(10000) % (Integer::from(10).pow(10)), 9947366875_u64);
    }
}

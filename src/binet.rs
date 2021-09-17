//! Implementation that computes terms of the Fibonacci sequence using the
//! closed form, Binet's formula. Uses arbitrary-precision floating-point
//! numbers, so depending on the given precision errors will start accumulating
//! after a while.

//! Implementation that converts the recursive definition of the Fibonacci
//! sequence to an iterative one, avoiding stack overflows and only computing
//! each value once.

use rug::{ops::Pow, Float, Integer};

use crate::fib_finder::FibFinder;

/// Dynamic programming approach: iterates through all of the sequence to reach the given target.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Binet {
    /// The precision to use. Higher values are slower but are correct for more values.
    prec: u32,
}

impl Default for Binet {
    fn default() -> Self {
        Self { prec: 10000 }
    }
}

impl FibFinder for Binet {
    fn fib(&mut self, n: u64) -> Integer {
        let one_half = Float::with_val(self.prec, 0.5);
        let sqrt5 = Float::with_val(self.prec, 5).sqrt();
        let phi = Float::with_val(self.prec, &one_half + &one_half * &sqrt5);
        let ans = Float::with_val(self.prec, phi.pow(n) / sqrt5);
        ans.to_integer().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let mut alg = Binet::default();
        assert_eq!(alg.fib(0), 0);
        assert_eq!(alg.fib(1), 1);
        assert_eq!(alg.fib(2), 1);
        assert_eq!(alg.fib(12), 144);
        assert_eq!(alg.fib(37), 24157817);

        assert_eq!(
            alg.fib(100),
            "354224848179261915075".parse::<Integer>().unwrap()
        );

        assert_eq!(
            alg.fib(1000),
            ("434665576869374564356885276750406258025646605173717804024817290895365554".to_owned()
                + "1794905189040387984007925516929592259308032263477520968962323987332247116164299"
                + "6440906533187938298969649928516003704476137795166849228875")
                .parse::<Integer>()
                .unwrap()
        );

        assert_eq!(alg.fib(10000) % (Integer::from(10).pow(10)), 9947366875_u64);
    }
}

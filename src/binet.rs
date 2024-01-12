//! Implementation that computes terms of the Fibonacci sequence using the
//! closed form, Binet's formula. Uses arbitrary-precision floating-point
//! numbers, so depending on the given precision errors will start accumulating
//! after a while.

use rug::{ops::Pow, Float, Integer};

use crate::{fib_finder::FibFinder, repeated_squaring::power};

/// Approach using Binet's formula.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Binet {}

impl FibFinder for Binet {
    fn fib(&mut self, n: u64) -> Integer {
        // using 1% more than necessary as a buffer
        let prec: u32 = (1.05 * (n as f64) * Float::with_val(10, 1.618).log2())
            .ceil()
            .to_integer()
            .unwrap()
            .max(Integer::from(53))
            .to_u32()
            .unwrap()
            + 10;
        let one_half = Float::with_val(prec, 0.5);
        let sqrt5 = Float::with_val(prec, 5).sqrt();
        let phi = Float::with_val(prec, &one_half + &one_half * &sqrt5);
        let ans = power(phi, n, Float::with_val(prec, 1)) / sqrt5;
        ans.to_integer().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_binet() {
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

        assert_eq!(
            alg.fib(100_000) % (Integer::from(10).pow(10)),
            3428746875_u64
        );
    }
}

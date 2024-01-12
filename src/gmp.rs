//! This module punts on computing Fibonacci numbers ourselves and simply wraps
//! GMP's algorithm. It's similar to the Cassini approach, but with a couple
//! more optimizations. https://gmplib.org/manual/Fibonacci-Numbers-Algorithm

use crate::fib_finder::FibFinder;
use rug::Integer;

/// Wraps GMP.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct GMP {}

impl FibFinder for GMP {
    fn fib(&mut self, n: u64) -> Integer {
        Integer::fibonacci(n as u32).into()
    }
}

#[cfg(test)]
mod tests {
    use rug::ops::Pow;

    use super::*;

    #[test]
    fn test_fib() {
        let mut alg = GMP::default();
        assert_eq!(alg.fib(0), 0u32);
        assert_eq!(alg.fib(1), 1u32);
        assert_eq!(alg.fib(2), 1u32);
        assert_eq!(alg.fib(12), 144u32);
        assert_eq!(alg.fib(37), 24157817u64);

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

        assert_eq!(
            alg.fib(1_000_000) % (Integer::from(10).pow(10)),
            8242546875_u64
        );

        assert_eq!(
            alg.fib(10_000_000) % (Integer::from(10).pow(10)),
            6380546875_u64
        );

        assert_eq!(
            alg.fib(100_000_000) % (Integer::from(10).pow(10)),
            7760546875_u64
        );

        assert_eq!(
            alg.fib(1_000_000_000) % (Integer::from(10).pow(10)),
            1560546875_u64
        );

        // this is out of the u32 limit
        // assert_eq!(
        //     alg.fib(10_000_000_000) % (Integer::from(10).pow(10)),
        //     9560546875_u64
        // );
    }
}

//! Implementation that converts the recursive definition of the Fibonacci
//! sequence to an iterative one, avoiding stack overflows and only computing
//! each value once.

use rug::Integer;

use crate::fib_finder::FibFinder;

/// Dynamic programming approach: iterates through all of the sequence to reach the given target.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct DPIterator {}

impl FibFinder for DPIterator {
    fn fib(&mut self, n: u64) -> Integer {
        match n {
            0 => Integer::from(0),
            1 => Integer::from(1),
            _ => {
                let mut prev = Integer::from(0);
                let mut curr = Integer::from(1);
                let mut new;
                for _ in 1..n {
                    new = curr.clone() + prev;
                    prev = curr;
                    curr = new;
                }
                curr
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rug::ops::Pow;

    use super::*;

    #[test]
    fn test_fib() {
        let mut alg = DPIterator::default();
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

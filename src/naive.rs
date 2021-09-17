//! Naïve approach to computing Fibonacci numbers: simple recursion.
use rug::Integer;

use crate::fib_finder::FibFinder;

/// Naïve recursive approach, using the basic definition of Fibonacci numbers: F(0) = 0, F(1) = 1, and F(n) = F(n - 1) + F(n - 2).
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct NaiveRecursor {}

impl FibFinder for NaiveRecursor {
    fn fib(&mut self, n: u64) -> Integer {
        match n {
            0 => Integer::from(0),
            1 => Integer::from(1),
            _ => self.fib(n - 1) + self.fib(n - 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let mut alg = NaiveRecursor::default();
        assert_eq!(alg.fib(0), 0);
        assert_eq!(alg.fib(1), 1);
        assert_eq!(alg.fib(2), 1);
        assert_eq!(alg.fib(12), 144);

        // even this takes a second
        // assert_eq!(alg.fib(37), 24157817);

        // This will take a while...

        // assert_eq!(
        //     alg.fib(100),
        //     "354224848179261915075".parse::<Integer>().unwrap()
        // );

        // Uh-oh! This will take a long, long time...

        // assert_eq!(
        //     alg.fib(1000),
        //     ("434665576869374564356885276750406258025646605173717804024817290895365554".to_owned()
        //         + "1794905189040387984007925516929592259308032263477520968962323987332247116164299"
        //         + "6440906533187938298969649928516003704476137795166849228875")
        //         .parse::<Integer>()
        //         .unwrap()
        // );
    }
}

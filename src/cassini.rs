//! Fibonacci algorithm similar in spirit to the matrix exponentiation approach,
//! but cutting out excess steps. The result is a set of two recursions that are
//! derived from Cassini's identity. The same addition chain approach used in
//! exponentiation by squaring can then be adapted to get to any number using
//! these two recursions.

use rug::Complete;
use rug::Integer;

use crate::fib_finder::FibFinder;

/// Cassini's identity recursion approach.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Cassini {}

impl FibFinder for Cassini {
    fn fib(&mut self, n: u64) -> rug::Integer {
        if n < 2 {
            return n.into();
        }

        let bits = format!("{:b}", n);

        let mut i = 1;

        let mut f_i = Integer::from(1u32);
        let mut f_iplus1 = Integer::from(1u32);

        let two = Integer::from(2u32);

        for b in bits.chars().skip(1) {
            let f_i_sqr = f_i.square_ref().complete();
            let f_i_iplus1 = f_i * &f_iplus1;
            let f_iplus1_sqr = f_iplus1.square();
            let f_2iplus1 = (&f_i_sqr + &f_iplus1_sqr).complete();

            let double_f_i_iplus1 = f_i_iplus1 * &two;
            if b == '0' {
                i = 2 * i;
                (f_i, f_iplus1) = (double_f_i_iplus1 - f_i_sqr, f_2iplus1);
            } else {
                i = 2 * i + 1;
                (f_i, f_iplus1) = (f_2iplus1, double_f_i_iplus1 + f_iplus1_sqr);
            }
        }

        assert!(i == n);

        f_i
    }
}

#[cfg(test)]
mod tests {
    use rug::ops::Pow;

    use super::*;

    #[test]
    fn test_fib() {
        let mut alg = Cassini::default();
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

        // assert_eq!(
        //     alg.fib(1_000_000_000) % (Integer::from(10).pow(10)),
        //     1560546875_u64
        // );

        // assert_eq!(
        //     alg.fib(10_000_000_000) % (Integer::from(10).pow(10)),
        //     9560546875_u64
        // );
    }
}

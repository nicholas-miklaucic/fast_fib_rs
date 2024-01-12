//! Basically a direct port of the GMP library's Fibonacci function, an
//! optimized version of the Cassini approach.

use rug::{Complete, Integer};

use crate::fib_finder::FibFinder;

/// Cassini's identity recursion approach.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct CassiniGMP {}

impl FibFinder for CassiniGMP {
    fn fib(&mut self, n: u64) -> rug::Integer {
        if n < 2 {
            return n.into();
        }

        let bits = format!("{:b}", n);

        let mut i = 1;
        let mut f_i = Integer::from(1u32);
        let mut f_im1 = Integer::from(0u32);

        let mut next_offset = -2i8;

        for b in bits.chars().skip(1) {
            let f_i_sqr = f_i.square_ref().complete();
            let f_im1_sqr = f_im1.square_ref().complete();
            // F[2i-1] = F[i]^2 + F[i-1]^2
            let f_2im1 = (&f_i_sqr + &f_im1_sqr).complete();
            // F[2i+1] = 4 F[k]^2 - F[i-1]^2 + 2*(-1)^i
            let f_2ip1 = (f_i_sqr << 2u32) - f_im1_sqr + next_offset;
            // F[2i] = F[2i+1] - F[2i-1]
            let f_2i = (&f_2ip1 - &f_2im1).complete();
            if b == '0' {
                i = 2 * i;
                (f_i, f_im1) = (f_2i, f_2im1);
                next_offset = 2;
            } else {
                i = 2 * i + 1;
                (f_i, f_im1) = (f_2ip1, f_2i);
                next_offset = -2;
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
        let mut alg = CassiniGMP::default();
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

        // assert_eq!(
        //     alg.fib(10_000_000_000) % (Integer::from(10).pow(10)),
        //     9560546875_u64
        // );
    }
}

//! Implementation of Binet's formula using integers for as much as possible,
//! working in the field extension Z(sqrt 5). There's some bookkeeping to avoid
//! unnecessary divisions by 2.

use std::{
    fmt::Display,
    ops::{Mul, MulAssign},
};

use crate::{repeated_squaring::power, FibFinder};
use rug::{Assign, Complete, Integer};

/// A number of the form a/2 + b/2 sqrt 5, with a and b integers.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Z5 {
    a: Integer,
    b: Integer,
}

impl Mul for Z5 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new = self.clone();
        new *= rhs;
        new
    }
}

impl MulAssign for Z5 {
    fn mul_assign(&mut self, rhs: Self) {
        // trick used here is like Karatsuba multiplication: we can save a big multiplication
        // we want to multiply (a + b root 5)(c + d root 5)
        // instead of returning (ac + 5bd) + (ad + bc) root 5
        // we do this
        // k1 = c(a + b)
        // k2 = b(c - 5d)
        // k3 = a(d - c)
        // ac + 5bd = k1 - k2
        // ad + bc = k1 + k3
        let (a, b) = (&self.a, &self.b);
        let (c, d) = (&rhs.a, &rhs.b);
        let k1 = c * (a + b).complete();
        let k2 = b * (c - 5u8 * d).complete();
        let k3 = a * (d - c).complete();

        // dbg!((a, b));
        // dbg!((c, d));
        // dbg!(&k1);
        // dbg!(&k2);
        // dbg!(&k3);

        self.a.assign(&k1 - k2);
        self.b.assign(&k1 + k3);

        // because it's really a/2 + b/2 root 5, and our new values have a 4 in
        // the denominator, we divide by 2
        self.a >>= 1;
        self.b >>= 1;
    }
}

impl Display for Z5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}√5", self.a, self.b)
    }
}

impl Z5 {
    /// Constructor
    pub fn new(a: u64, b: u64) -> Z5 {
        Z5 {
            a: a.into(),
            b: b.into(),
        }
    }

    /// The multiplicative identity.
    pub fn one() -> Z5 {
        Z5::new(2, 0)
    }
}

/// Binet approach using Z(root 5) integer field extension.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct BinetZ5 {}

impl FibFinder for BinetZ5 {
    fn fib(&mut self, n: u64) -> Integer {
        match n {
            0 => 0.into(),
            1 => 1.into(),
            _ => {
                // we want to compute the rounded version of phi^n / sqrt 5
                // represent as (Z5{1, 1})^n - Z5({1, -1})^n) / sqrt5
                // we don't need to compute sqrt(5): the answer will just be the root 5 part over 2
                let ans = power(Z5::new(1, 1), n, Z5::one());
                ans.b
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
        let mut alg = BinetZ5::default();
        // assert_eq!(alg.fib(0), 0u64);
        assert_eq!(alg.fib(1), 1u64);
        assert_eq!(alg.fib(2), 1u64);
        assert_eq!(alg.fib(12), 144u64);
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

        assert_eq!(
            alg.fib(10000) % (Integer::from(10u64).pow(10u32)),
            9947366875_u64
        );

        assert_eq!(
            alg.fib(100_000) % (Integer::from(10u64).pow(10u32)),
            3428746875_u64
        );

        assert_eq!(
            alg.fib(1_000_000) % (Integer::from(10u64).pow(10u32)),
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

        // this is out of the u32 limit
        // assert_eq!(
        //     alg.fib(10_000_000_000) % (Integer::from(10).pow(10)),
        //     9560546875_u64
        // );
    }
}

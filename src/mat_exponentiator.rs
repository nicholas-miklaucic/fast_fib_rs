//! Implementation that uses matrix multiplication and repeated squaring to
//! avoid some intermediate numbers in the sequence instead of computing the
//! entire sequence. Unpacks all matrix multiplications to avoid relying on a
//! linear algebra library and reduce overhead.

use std::ops::{Mul, MulAssign};

use rug::Integer;

use crate::{fib_finder::FibFinder, repeated_squaring::power};

/// A simple 2x2 matrix with inlined multiplication and big integers.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Mat2x2 {
    /// The top left element.
    a: Integer,
    /// The top right element.
    b: Integer,
    /// The bottom left element.
    c: Integer,
    /// The bottom right element.
    d: Integer,
}

impl Mat2x2 {
    /// Returns the identity element.
    pub fn identity() -> Self {
        return Mat2x2 {
            a: 1.into(),
            b: 0.into(),
            c: 0.into(),
            d: 1.into(),
        };
    }
}

impl Mul<Mat2x2> for Mat2x2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // (a b) (a' b') => (aa' + bc' ab' + bd')
        // (c d) (c' d') => (ca' + dc' cb' + dd')
        return Mat2x2 {
            a: Integer::from(&self.a * &rhs.a) + Integer::from(&self.b * &rhs.c),
            b: Integer::from(&self.a * &rhs.b) + Integer::from(&self.b * &rhs.d),
            c: Integer::from(&self.c * &rhs.a) + Integer::from(&self.d * &rhs.c),
            d: Integer::from(&self.c * &rhs.b) + Integer::from(&self.d * &rhs.d),
        };
    }
}

impl MulAssign<Mat2x2> for Mat2x2 {
    fn mul_assign(&mut self, rhs: Mat2x2) {
        *self = self.clone() * rhs;
    }
}

impl Mul<(Integer, Integer)> for Mat2x2 {
    type Output = (Integer, Integer);

    fn mul(self, rhs: (Integer, Integer)) -> Self::Output {
        let (x, y) = rhs;
        (
            Integer::from(&self.a * &x) + Integer::from(&self.b * &y),
            Integer::from(&self.c * &x) + Integer::from(&self.d * &y),
        )
    }
}

/// Matrix exponentiation approach using repeated squaring.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct MatExponentiator {}

impl FibFinder for MatExponentiator {
    fn fib(&mut self, n: u64) -> Integer {
        let fib_mat = Mat2x2 {
            a: 1.into(),
            b: 1.into(),
            c: 1.into(),
            d: 0.into(),
        };
        // dbg!(power(fib_mat.clone(), 3, Mat2x2::identity()));
        let (fib_curr, _fib_prev) = power(fib_mat, n, Mat2x2::identity()) * (0.into(), 1.into());
        fib_curr
    }
}

#[cfg(test)]
mod tests {
    use rug::ops::Pow;

    use super::*;

    #[test]
    fn test_fib() {
        let mut alg = MatExponentiator::default();
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

        assert_eq!(
            alg.fib(1_000_000) % (Integer::from(10).pow(10)),
            8242546875_u64
        );
    }
}

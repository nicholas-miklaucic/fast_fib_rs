//! Implements a generalized exponentiation algorithm that accepts any group elements, computing powers using repeated squaring.

use std::ops::MulAssign;

/// Raises base to power exp. ident is x^0 for any x and the identity element
/// under the group operation.
pub fn power<T: Clone + MulAssign>(base: T, exp: u64, ident: T) -> T {
    if exp == 0 {
        return ident;
    }
    let bits = format!("{:b}", exp);
    let mut p = base.clone();
    let mut prod = ident;
    for b in bits.chars().rev() {
        if b == '1' {
            prod *= p.clone();
        }
        p *= p.clone();
    }

    return prod;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_pow() {
        let bases: Vec<u64> = vec![3, 4, 2, 5, 10, 6];
        let exps: Vec<u32> = vec![8, 10, 17, 5, 1, 0];
        for (base, exp) in bases.into_iter().zip(exps) {
            assert_eq!(power(base, exp.into(), 1), base.pow(exp))
        }
    }
}

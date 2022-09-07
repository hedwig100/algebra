use super::field;
use super::gcd;

use field::Field;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fp<const P: i32> {
    val: i32,
}

impl<const P: i32> Fp<P> {
    pub fn new(val: i32) -> Fp<P> {
        Fp::<P> { val: val % P }
    }
}

impl<const P: i32> Add for Fp<P> {
    type Output = Self;
    fn add(self, rhs: Fp<P>) -> Self {
        Fp::<P> {
            val: (self.val + rhs.val) % P,
        }
    }
}

impl<const P: i32> Sub for Fp<P> {
    type Output = Fp<P>;
    fn sub(self, rhs: Fp<P>) -> Self::Output {
        Fp::<P> {
            val: (self.val + P - rhs.val) % P,
        }
    }
}

impl<const P: i32> Mul for Fp<P> {
    type Output = Fp<P>;
    fn mul(self, rhs: Fp<P>) -> Self::Output {
        Fp::<P> {
            val: (self.val * rhs.val) % P,
        }
    }
}

impl<const P: i32> Div for Fp<P> {
    type Output = Fp<P>;
    fn div(self, rhs: Fp<P>) -> Self::Output {
        let (mut x, _) = gcd::ext_gcd(rhs.val, P);
        x = ((x % P) + P) % P;
        Fp::<P> {
            val: (self.val * x) % P,
        }
    }
}

impl<const P: i32> Neg for Fp<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fp::<P> {
            val: (P - self.val) % P,
        }
    }
}

impl<const P: i32> Field for Fp<P> {
    fn unit() -> Fp<P> {
        Fp::<P> { val: 1 }
    }
    fn zero() -> Fp<P> {
        Fp::<P> { val: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fp_test() {
        let x = Fp::<7>::new(2);
        let y = Fp::<7>::new(10);
        let add = x + y;
        assert_eq!(add.val, 5);
        let minus = x - y;
        assert_eq!(minus.val, 6);
        let mul = x * y;
        assert_eq!(mul.val, 6);
        let div = x / y;
        assert_eq!(div.val, 3);
    }
}

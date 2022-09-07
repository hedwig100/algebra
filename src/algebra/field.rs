use std::fmt::Debug;
use std::ops::Neg;
use std::ops::{Add, Div, Mul, Sub};

pub trait Field:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + PartialEq
    + Eq
    + Sized
    + Copy
    + Debug
{
    fn unit() -> Self;
    fn zero() -> Self;
}

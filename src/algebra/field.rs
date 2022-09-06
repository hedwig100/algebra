use core::ops::{Add, Div, Mul, Sub};

pub trait Field:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sized
    + Clone
    + Copy
{
    fn unit() -> Self;
    fn zero() -> Self;
}

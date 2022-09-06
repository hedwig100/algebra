use std::cmp::Ordering;

use crate::algebra::field;

#[derive(Clone, Copy)]
pub struct Monomial<F, const N: usize>
where
    F: field::Field,
{
    pub coef: F,
    pub degree: [u32; N],
}

impl<F, const N: usize> Monomial<F, N>
where
    F: field::Field,
{
    pub fn new(coef: F, degree: [u32; N]) -> Monomial<F, N> {
        Monomial { coef, degree }
    }

    pub fn can_divide(&self, rhs: &Monomial<F, N>) -> bool {
        for i in 0..N {
            if self.degree[i] < rhs.degree[i] {
                return false;
            }
        }
        return true;
    }

    pub fn minus(&self, rhs: &Monomial<F, N>) -> Monomial<F, N> {
        Monomial {
            coef: self.coef - rhs.coef,
            degree: self.degree,
        }
    }

    // divide
    // rhsで割る, ただし割ることを確認してから呼ぶこと
    pub fn divide(&self, rhs: &Monomial<F, N>) -> Monomial<F, N> {
        let coef = self.coef / rhs.coef;
        let mut degree = [0; N];
        for (j, deg) in degree.iter_mut().enumerate() {
            *deg = self.degree[j] - rhs.degree[j];
        }
        Monomial { coef, degree }
    }
}

impl<F, const N: usize> PartialEq for Monomial<F, N>
where
    F: field::Field,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.degree[i] != other.degree[i] {
                return false;
            }
        }
        true
    }
}

impl<F, const N: usize> Eq for Monomial<F, N> where F: field::Field {}

impl<F, const N: usize> PartialOrd for Monomial<F, N>
where
    F: field::Field,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in 0..N {
            match self.degree[i].cmp(&other.degree[i]) {
                Ordering::Equal => continue,
                x => return Some(x),
            }
        }
        Some(Ordering::Equal)
    }
}

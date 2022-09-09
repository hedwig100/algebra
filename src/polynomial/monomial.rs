use std::cmp::max;
use std::cmp::Ordering;
use std::fmt;

use crate::algebra::field;

// Monomial
// 係数体をFとしてN変数の単項式を表す構造体, ここでは係数を含めて単項式と呼ぶことにする.
#[derive(Clone, Debug)]
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

    pub fn is_zero(&self) -> bool {
        self.coef == F::zero()
    }

    // neg
    // -selfを返す.
    pub fn neg(&self) -> Monomial<F, N> {
        Monomial {
            coef: -self.coef,
            degree: self.degree,
        }
    }

    // sub
    // 単項式を引く, ただし次数が同じ場合であるのみ呼べる.
    pub fn sub(&self, rhs: &Monomial<F, N>) -> Monomial<F, N> {
        Monomial {
            coef: self.coef - rhs.coef,
            degree: self.degree,
        }
    }

    pub fn can_divide(&self, rhs: &Monomial<F, N>) -> bool {
        for i in 0..N {
            if self.degree[i] < rhs.degree[i] {
                return false;
            }
        }
        return true;
    }

    // div
    // rhsで割る, ただし割れることを確認してから呼ぶこと
    pub fn div(&self, rhs: &Monomial<F, N>) -> Monomial<F, N> {
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
    // eq
    // 次数が等しい時に等しいと判定している.
    // 係数は等しくなくても等しいと判定されることに注意する.
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
    // partial_cmp
    // 辞書順で単項式を比較する.
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

impl<F, const N: usize> fmt::Display for Monomial<F, N>
where
    F: field::Field + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let deg = self
            .degree
            .iter()
            .enumerate()
            .filter(|(_, deg)| **deg != 0)
            .map(|(i, deg)| format!("x_{}^{}", i + 1, deg))
            .collect::<String>();
        if !deg.is_empty() && self.coef == F::unit() {
            write!(f, "{}", deg)
        } else {
            write!(f, "{}{}", self.coef, deg)
        }
    }
}

pub fn lcm<F, const N: usize>(mono1: &Monomial<F, N>, mono2: &Monomial<F, N>) -> Monomial<F, N>
where
    F: field::Field,
{
    let mut degree = [0; N];
    for (i, deg) in degree.iter_mut().enumerate() {
        *deg = max(mono1.degree[i], mono2.degree[i]);
    }
    Monomial::new(mono1.coef * mono2.coef, degree)
}

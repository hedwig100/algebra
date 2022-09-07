use super::monomial::Monomial;
use crate::algebra::field;

#[derive(Debug)]
pub struct Polynomial<F, const N: usize>
where
    F: field::Field,
{
    pub monos: Vec<Monomial<F, N>>,
}

impl<F, const N: usize> Polynomial<F, N>
where
    F: field::Field,
{
    pub fn new(monos: Vec<Monomial<F, N>>) -> Polynomial<F, N> {
        Polynomial { monos }
    }

    pub fn is_zero(&self) -> bool {
        self.monos.is_empty()
    }

    // sub
    // rhsを引いた結果を返す.
    pub fn sub(&self, rhs: &Polynomial<F, N>) -> Polynomial<F, N> {
        let l_len = self.monos.len();
        let r_len = rhs.monos.len();
        let mut monos = Vec::with_capacity(l_len + r_len);
        let mut li = 0;
        let mut ri = 0;
        while li != l_len || ri != r_len {
            if li == l_len {
                monos.push(rhs.monos[ri].neg());
                ri += 1;
            } else if ri == r_len {
                monos.push(self.monos[li].clone());
                li += 1;
            } else if self.monos[li] == rhs.monos[ri] {
                let x = self.monos[li].sub(&rhs.monos[ri]);
                if !x.is_zero() {
                    monos.push(x);
                }
                li += 1;
                ri += 1;
            } else if self.monos[li] > rhs.monos[ri] {
                monos.push(self.monos[li].clone());
                li += 1;
            } else {
                monos.push(rhs.monos[ri].neg());
                ri += 1;
            }
        }
        Polynomial { monos }
    }

    // mul
    // 単項式をかける.
    pub fn mul(&self, rhs: &Monomial<F, N>) -> Polynomial<F, N> {
        let mut monos: Vec<Monomial<F, N>> = Vec::with_capacity(self.monos.len());
        for i in 0..self.monos.len() {
            let mut degree = [0; N];
            for (j, deg) in degree.iter_mut().enumerate() {
                *deg = self.monos[i].degree[j] + rhs.degree[j];
            }
            monos.push(Monomial::new(self.monos[i].coef * rhs.coef, degree));
        }
        Polynomial { monos }
    }

    // simplify
    // 多項式を多項式で簡約化する. 簡約化できた場合はtrueを返し, 簡約化できない場合はfalseを返す.
    pub fn simplify(&mut self, rhs: &Polynomial<F, N>) -> bool {
        if rhs.monos.is_empty() {
            return false;
        }
        for mono in self.monos.iter() {
            match mono.can_divide(&rhs.monos[0]) {
                false => continue,
                true => {
                    let rhs = rhs.mul(&mono.div(&rhs.monos[0]));
                    eprintln!("{:?}", rhs);
                    *self = (*self).sub(&rhs);
                    return true;
                }
            }
        }
        false
    }
}

impl<F, const N: usize> PartialEq for Polynomial<F, N>
where
    F: field::Field,
{
    fn eq(&self, other: &Self) -> bool {
        if self.monos.len() != other.monos.len() {
            return false;
        }
        for i in 0..self.monos.len() {
            if self.monos[i] != other.monos[i] || self.monos[i].coef != other.monos[i].coef {
                return false;
            }
        }
        true
    }
}

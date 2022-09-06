use super::monomial::Monomial;
use crate::algebra::field;

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

    pub fn minus(&self, rhs: &Polynomial<F, N>) -> Polynomial<F, N> {
        let l_len = self.monos.len();
        let r_len = rhs.monos.len();
        let mut monos = Vec::with_capacity(l_len + r_len);
        let mut li = 0;
        let mut ri = 0;
        while li != l_len && ri != r_len {
            if li == l_len {
                monos.push(rhs.monos[ri]);
                ri += 1;
            } else if ri == r_len {
                monos.push(self.monos[li]);
                li += 1;
            } else if self.monos[li] == rhs.monos[ri] {
                monos.push(self.monos[li].minus(&rhs.monos[ri]));
                li += 1;
                ri += 1;
            } else if self.monos[li] > rhs.monos[ri] {
                monos.push(rhs.monos[ri]);
                ri += 1;
            } else {
                monos.push(self.monos[li]);
                li += 1;
            }
        }
        Polynomial { monos }
    }

    // mul
    // 単項式をかける. 割れることを確認してから呼ぶこと
    pub fn mul(&self, rhs: &Monomial<F, N>) -> Polynomial<F, N> {
        let mut monos: Vec<Monomial<F, N>> = Vec::with_capacity(self.monos.len());
        for i in 0..self.monos.len() {
            let mut degree = [0; N];
            for (j, deg) in degree.iter_mut().enumerate() {
                *deg = self.monos[i].degree[j] - rhs.degree[j];
            }
            monos[i] = Monomial {
                coef: self.monos[i].coef / rhs.coef,
                degree,
            };
        }
        Polynomial { monos }
    }

    pub fn simplify(&mut self, mono: &Monomial<F, N>) -> bool {
        if self.monos.is_empty() || !mono.can_divide(&self.monos[0]) {
            return false;
        }
        let rhs = self.mul(&mono.divide(&self.monos[0]));
        *self = (*self).minus(&rhs);
        return true;
    }
}

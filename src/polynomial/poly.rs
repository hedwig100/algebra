use super::monomial::Monomial;
use crate::algebra::field;

// Polynomial
// 係数体をFとしてN変数の多項式を表す構造体.
// monosは単項式の順序を保つ, すなわち単項式順序で大きいものから並ぶ.
// すべての演算の結果は単項式の順序を保つようなものではなくてはならない.
#[derive(Debug, Clone)]
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
        if rhs.is_zero() {
            return false;
        }
        for mono in self.monos.iter() {
            match mono.can_divide(&rhs.monos[0]) {
                false => continue,
                true => {
                    let rhs = rhs.mul(&mono.div(&rhs.monos[0]));
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

// simplify
// 多項式polyを多項式の集合polysで簡約化する.
// もし簡約化できなければfalseとpolyをそのまま返す, 簡約化できれば簡約化したものを返す.
// polyをpolysで簡約化するとは, polyをpolysに含まれる多項式で簡約化し, 任意の項がpolysに含まれる任意の多項式の
// 先頭多項式で割り切れないようにすることをいう.
pub fn simplify<F, const N: usize>(
    mut poly: Polynomial<F, N>,
    polys: &[Polynomial<F, N>],
) -> (bool, Polynomial<F, N>)
where
    F: field::Field,
{
    let mut simplifed_once = false;
    let mut is_simplified = true;
    while is_simplified {
        is_simplified = false;
        for rhs in polys.iter() {
            is_simplified |= poly.simplify(rhs);
            if is_simplified {
                simplifed_once = true;
                break;
            }
        }
    }
    (simplifed_once, poly)
}

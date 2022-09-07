use crate::algebra::field;
use crate::polynomial::monomial;
use crate::polynomial::poly::Polynomial;

// s
// f, gのS多項式を求める. f,gいずれも0でないことを確認してから呼ぶこと.
pub fn s_poly<F, const N: usize>(f: &Polynomial<F, N>, g: &Polynomial<F, N>) -> Polynomial<F, N>
where
    F: field::Field,
{
    let lcm = monomial::lcm(&f.monos[0], &g.monos[0]);
    f.mul(&lcm.div(&f.monos[0]))
        .sub(&g.mul(&lcm.div(&g.monos[0])))
}

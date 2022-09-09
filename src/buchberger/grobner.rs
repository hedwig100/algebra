use crate::polynomial::poly::Polynomial;

use super::s_polynomial;
use crate::algebra::field;
use crate::polynomial::poly;

// buchberger
// polysで生成される多項式環のイデアルのグレブナー基底を求める.
pub fn buchberger<F, const N: usize>(mut polys: Vec<Polynomial<F, N>>) -> Vec<Polynomial<F, N>>
where
    F: field::Field,
{
    loop {
        let mut push_poly = None;
        for f in polys.iter() {
            for g in polys.iter() {
                if f == g {
                    continue;
                }
                let (_, s) = poly::simplify(s_polynomial::s_poly(f, g), &polys);
                if !s.is_zero() {
                    push_poly = Some(s);
                    break;
                }
            }
            if push_poly.is_some() {
                break;
            }
        }
        match push_poly {
            Some(s) => polys.push(s),
            None => break,
        }
    }
    polys
}

// minimal_grobner
// グレブナー基底から極小グレブナー基底を求める.
pub fn minimal_grobner<F, const N: usize>(grobner: Vec<Polynomial<F, N>>) -> Vec<Polynomial<F, N>>
where
    F: field::Field,
{
    let mut pick_index = vec![];
    for (i, poly) in grobner.iter().enumerate() {
        let mut can_div = false;
        for f in grobner.iter() {
            if poly != f && poly.monos[0].can_divide(&f.monos[0]) {
                can_div = true;
                break;
            }
        }
        if !can_div {
            pick_index.push(i);
        }
    }
    grobner
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| pick_index.binary_search(&i).is_ok())
        .map(|(_, mut poly)| {
            poly.norm();
            poly
        })
        .collect()
}

fn simplify_except_myself<F, const N: usize>(
    mut poly: Polynomial<F, N>,
    polys: &[Polynomial<F, N>],
    myself: usize,
) -> Polynomial<F, N>
where
    F: field::Field,
{
    let mut is_simplified = true;
    while is_simplified {
        is_simplified = false;
        for (i, rhs) in polys.iter().enumerate() {
            if i == myself {
                continue;
            }
            is_simplified |= poly.simplify(rhs);
            if is_simplified {
                break;
            }
        }
    }
    poly
}

// simplified_grobner
// 極小グレブナー基底から簡約グレブナー基底を求める.
pub fn simplified_grobner<F, const N: usize>(
    grobner: Vec<Polynomial<F, N>>,
) -> Vec<Polynomial<F, N>>
where
    F: field::Field,
{
    let mut ans = Vec::with_capacity(grobner.len());
    for (i, poly) in grobner.iter().enumerate() {
        let poly = simplify_except_myself(poly.clone(), &grobner, i);
        ans.push(poly);
    }
    ans
}

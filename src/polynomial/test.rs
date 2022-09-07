use super::{monomial::Monomial, poly::Polynomial};
use crate::algebra::fp;

type Fp = fp::Fp<7>;

#[test]
fn monomial() {
    let mono1 = Monomial::new(Fp::new(3), [2, 3, 3]);
    let mono2 = Monomial::new(Fp::new(5), [1, 2, 3]);

    assert!(mono1.can_divide(&mono2));
    assert_eq!(mono1.div(&mono2), Monomial::new(Fp::new(2), [1, 1, 0]));

    let mono3 = Monomial::new(Fp::new(8), [2, 3, 3]);
    assert_eq!(mono1.sub(&mono3), Monomial::new(Fp::new(2), [2, 3, 3]));
}

#[test]
fn polynomial() {
    let monos1 = vec![
        Monomial::new(Fp::new(4), [3, 1, 2]),
        Monomial::new(Fp::new(1), [1, 2, 0]),
        Monomial::new(Fp::new(1), [0, 1, 5]),
    ];
    let monos2 = vec![
        Monomial::new(Fp::new(3), [5, 1, 0]),
        Monomial::new(Fp::new(4), [1, 2, 0]),
    ];
    let poly1 = Polynomial::new(monos1);
    let poly2 = Polynomial::new(monos2);
    let mut poly3 = poly1.sub(&poly2);
    assert_eq!(
        poly3,
        Polynomial::new(vec![
            Monomial::new(Fp::new(4), [5, 1, 0]),
            Monomial::new(Fp::new(4), [3, 1, 2]),
            Monomial::new(Fp::new(4), [1, 2, 0]),
            Monomial::new(Fp::new(1), [0, 1, 5]),
        ])
    );

    let poly4 = Polynomial::new(vec![
        Monomial::new(Fp::new(5), [2, 1, 1]),
        Monomial::new(Fp::new(3), [1, 0, 0]),
    ]);
    assert!(poly3.simplify(&poly4));
    assert_eq!(
        poly3,
        Polynomial::new(vec![
            Monomial::new(Fp::new(4), [5, 1, 0]),
            Monomial::new(Fp::new(6), [2, 0, 1]),
            Monomial::new(Fp::new(4), [1, 2, 0]),
            Monomial::new(Fp::new(1), [0, 1, 5])
        ])
    )
}

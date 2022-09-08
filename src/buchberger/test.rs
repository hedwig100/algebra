use crate::algebra::fp;
use crate::buchberger::grobner::minimal_grobner;
use crate::polynomial::monomial::Monomial;
use crate::polynomial::poly::Polynomial;

use super::grobner::{buchberger, simplified_grobner};

type Fp = fp::Fp<5>;

#[test]
fn buch_test() {
    let ideal = vec![
        Polynomial::new(vec![
            Monomial::new(Fp::new(1), [2, 1]),
            Monomial::new(Fp::new(4), [0, 0]),
        ]),
        Polynomial::new(vec![
            Monomial::new(Fp::new(1), [1, 2]),
            Monomial::new(Fp::new(4), [1, 0]),
        ]),
    ];
    let mut grobner = buchberger(ideal);
    assert_eq!(
        grobner,
        vec![
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [2, 1]),
                Monomial::new(Fp::new(4), [0, 0]),
            ]),
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [1, 2]),
                Monomial::new(Fp::new(4), [1, 0]),
            ]),
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [2, 0]),
                Monomial::new(Fp::new(4), [0, 1]),
            ]),
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [0, 2]),
                Monomial::new(Fp::new(4), [0, 0]),
            ]),
        ]
    );
    grobner = minimal_grobner(grobner);
    assert_eq!(
        grobner,
        vec![
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [2, 0]),
                Monomial::new(Fp::new(4), [0, 1]),
            ]),
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [0, 2]),
                Monomial::new(Fp::new(4), [0, 0]),
            ]),
        ]
    );
}

#[test]
fn simple_grobner() {
    let mut grobner = vec![
        Polynomial::new(vec![
            Monomial::new(Fp::new(1), [2, 0]),
            Monomial::new(Fp::new(3), [0, 2]),
            Monomial::new(Fp::new(4), [0, 1]),
        ]),
        Polynomial::new(vec![
            Monomial::new(Fp::new(1), [0, 2]),
            Monomial::new(Fp::new(4), [0, 0]),
        ]),
    ];
    grobner = simplified_grobner(grobner);
    assert_eq!(
        grobner,
        vec![
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [2, 0]),
                Monomial::new(Fp::new(4), [0, 1]),
                Monomial::new(Fp::new(3), [0, 0])
            ]),
            Polynomial::new(vec![
                Monomial::new(Fp::new(1), [0, 2]),
                Monomial::new(Fp::new(4), [0, 0]),
            ]),
        ]
    )
}

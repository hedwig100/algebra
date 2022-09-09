use super::*;
use crate::algebra::fp;
use crate::polynomial::monomial::Monomial;
use crate::polynomial::poly::Polynomial;

const P: i32 = 5;
type Fp = fp::Fp<P>;

#[test]
fn test_parse() {
    let eq1 = "4x_1x_2^3 + 3x_2^4x_3^5 + x_1 + 4, x_1^2 + 2";

    match parser::parse(eq1) {
        Ok(poly) => {
            assert_eq!(poly.len(), 2);
            assert_eq!(
                poly[0],
                Polynomial::new(vec![
                    Monomial::new(Fp::new(4), [1, 3, 0]),
                    Monomial::new(Fp::new(3), [0, 4, 5]),
                    Monomial::new(Fp::new(1), [1, 0, 0]),
                    Monomial::new(Fp::new(4), [0, 0, 0]),
                ])
            );
            assert_eq!(
                poly[1],
                Polynomial::new(vec![
                    Monomial::new(Fp::new(1), [2, 0, 0]),
                    Monomial::new(Fp::new(2), [0, 0, 0]),
                ])
            );
        }
        Err(msg) => {
            eprintln!("{}", msg);
            panic!();
        }
    }
}

use algebra::algebra::fp;
use algebra::polynomial::monomial::Monomial;
use algebra::polynomial::poly::Polynomial;

type Fp = fp::Fp<5>;

fn main() {
    let x = Polynomial::new(vec![
        Monomial::new(Fp::new(3), [4, 1, 3]),
        Monomial::new(Fp::new(3), [1, 2, 3]),
    ]);
    println!("{}", x);
}

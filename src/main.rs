use algebra::{algebra::fp, buchberger::grobner, equation_parser::parser, polynomial::poly};
use clap::Parser;

const P: i32 = 7;
const N: usize = 3;

#[derive(Parser)]
struct Args {
    #[clap(short = 'e', long = "equation")]
    equation: String,
}

fn main() {
    let args = Args::parse();
    let polys: Vec<poly::Polynomial<fp::Fp<P>, N>> = match parser::parse(&args.equation) {
        Ok(polys) => polys,
        Err(msg) => panic!("{}", msg),
    };
    println!(
        "Original Ideal: ({})",
        polys
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(", ")
    );
    let grobner = grobner::buchberger(polys);
    let minimal = grobner::minimal_grobner(grobner);
    let simple = grobner::simplified_grobner(minimal);
    println!(
        "Simplified Grobner Basis: {}",
        simple
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(", ")
    );
}

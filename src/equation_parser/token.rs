#[derive(Debug)]
pub enum Token {
    Num(u32),
    Var(char),
    Symb(char),
    EOF,
}

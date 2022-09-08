pub enum Token {
    Num(u32),
    Var(char),
    Op(char),
    Symbol(char),
    EOF,
}

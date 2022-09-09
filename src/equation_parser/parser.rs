use super::lexer::Lexer;
use super::token::Token;
use crate::algebra::fp;
use crate::polynomial::monomial::Monomial;
use crate::polynomial::poly::Polynomial;

pub struct Parser {
    pub tokens: Vec<Token>,
    now: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, now: 0 }
    }

    fn read(&self) -> Option<&Token> {
        if self.now < self.tokens.len() {
            Some(&self.tokens[self.now])
        } else {
            None
        }
    }

    fn next(&mut self) {
        self.now += 1;
    }

    fn err<T>(&mut self, prev_id: usize, msg: &'static str) -> Result<T, &'static str> {
        self.now = prev_id;
        Err(msg)
    }

    pub fn parse<const P: i32, const N: usize>(
        &mut self,
    ) -> Result<Vec<Polynomial<fp::Fp<P>, N>>, &'static str> {
        let mut ans = Vec::new();
        loop {
            match self.poly::<P, N>() {
                Ok(poly) => ans.push(poly),
                Err(msg) => return Err(msg),
            };
            if let Some(Token::Symb(',')) = self.read() {
                self.next();
            } else {
                break;
            }
        }
        Ok(ans)
    }

    fn poly<const P: i32, const N: usize>(
        &mut self,
    ) -> Result<Polynomial<fp::Fp<P>, N>, &'static str> {
        let mut monos = Vec::new();
        loop {
            match self.term() {
                Ok(ter) => monos.push(ter),
                Err(msg) => return Err(msg),
            };
            match self.read() {
                Some(Token::Symb('+')) => self.next(),
                Some(Token::Symb('-')) => self.next(),
                _ => break,
            };
        }
        Ok(Polynomial::new(monos))
    }

    fn term<const P: i32, const N: usize>(
        &mut self,
    ) -> Result<Monomial<fp::Fp<P>, N>, &'static str> {
        let mut is_read = false;
        let coef = fp::Fp::<P>::new(if let Some(&Token::Num(num)) = self.read() {
            is_read = true;
            num as i32
        } else {
            1
        });
        if is_read {
            self.next();
        }
        match self.mono::<P, N>() {
            Ok(degree) => {
                if degree.iter().all(|&x| x == 0) && !is_read {
                    Err("empty term")
                } else {
                    Ok(Monomial::new(coef, degree))
                }
            }
            Err(msg) => Err(msg),
        }
    }

    fn mono<const P: i32, const N: usize>(&mut self) -> Result<[u32; N], &'static str> {
        let mut degree = [0; N];
        while let Some(&Token::Var('x')) = self.read() {
            match self.var() {
                Ok((num, deg)) => degree[(num - 1) as usize] += deg,
                Err(msg) => return Err(msg),
            }
        }
        Ok(degree)
    }

    fn var(&mut self) -> Result<(u32, u32), &'static str> {
        let prev_id = self.now;
        if let Some(&Token::Var('x')) = self.read() {
            self.next();
            if let Some(&Token::Symb('_')) = self.read() {
                self.next();
                if let Some(&Token::Num(num)) = self.read() {
                    self.next();
                    if let Some(&Token::Symb('^')) = self.read() {
                        match self.pow() {
                            Ok(deg) => Ok((num, deg)),
                            Err(msg) => Err(msg),
                        }
                    } else {
                        Ok((num, 1))
                    }
                } else {
                    self.err(prev_id, "'x_' is invalid")
                }
            } else {
                self.err(prev_id, "'x' is invalid")
            }
        } else {
            self.err(prev_id, "not variable")
        }
    }

    fn pow(&mut self) -> Result<u32, &'static str> {
        let prev_id = self.now;
        if let Some(&Token::Symb('^')) = self.read() {
            self.next();
            if let Some(&Token::Num(num)) = self.read() {
                self.next();
                Ok(num)
            } else {
                self.err(prev_id, "'^' is invalid")
            }
        } else {
            self.err(prev_id, "not pow")
        }
    }
}

pub fn parse<const P: i32, const N: usize>(
    eq: &str,
) -> Result<Vec<Polynomial<fp::Fp<P>, N>>, &'static str> {
    let mut lexer = Lexer::new(eq);
    match lexer.lex() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            parser.parse()
        }
        Err(msg) => Err(msg),
    }
}

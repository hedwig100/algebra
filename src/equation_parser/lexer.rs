use super::token::Token;
use std::str;

pub struct Lexer<'a> {
    pub sentence: &'a str,
    cursor: str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(sentence: &'a str) -> Lexer<'a> {
        Lexer {
            sentence,
            cursor: sentence.chars(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, &'static str> {
        let mut ans = Vec::new();
        let mut num = 0;
        while let Some(c) = self.cursor.next() {
            match self.lex_char(c) {
                Ok(Token::Num(a)) => {
                    num = num * 10 + a;
                }
                Ok(Token::Symb(' ')) => continue,
                Ok(tok) => {
                    self.add_num(&mut ans, &mut num);
                    ans.push(tok);
                }
                Err(msg) => {
                    self.add_num(&mut ans, &mut num);
                    return Err(msg);
                }
            }
        }
        self.add_num(&mut ans, &mut num);
        ans.push(Token::EOF);
        Ok(ans)
    }

    fn lex_char(&mut self, c: char) -> Result<Token, &'static str> {
        match c {
            '0' => Ok(Token::Num(0)),
            '1' => Ok(Token::Num(1)),
            '2' => Ok(Token::Num(2)),
            '3' => Ok(Token::Num(3)),
            '4' => Ok(Token::Num(4)),
            '5' => Ok(Token::Num(5)),
            '6' => Ok(Token::Num(6)),
            '7' => Ok(Token::Num(7)),
            '8' => Ok(Token::Num(8)),
            '9' => Ok(Token::Num(9)),
            'x' => Ok(Token::Var('x')),
            '^' => Ok(Token::Symb('^')),
            '+' => Ok(Token::Symb('+')),
            '-' => Ok(Token::Symb('-')),
            '_' => Ok(Token::Symb('_')),
            ',' => Ok(Token::Symb(',')),
            ' ' => Ok(Token::Symb(' ')),
            _ => Err("unexpected character"),
        }
    }

    fn add_num(&self, ans: &mut Vec<Token>, num: &mut u32) {
        if *num > 0 {
            (*ans).push(Token::Num(*num));
            *num = 0;
        }
    }
}

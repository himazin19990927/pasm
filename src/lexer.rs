use crate::token::*;
use core::panic;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'input> {
    chars: Chars<'input>,
    ch: Option<char>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            ch: None,
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        self.ch = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Token {
        match self.ch {
            Some(ch) => {
                if !ch.is_digit(10) {
                    panic!("A non-numeric value was entered")
                }
            }
            None => panic!("Entered string has already reached the end."),
        }

        let mut digits = String::from(self.ch.unwrap());
        loop {
            self.read_char();
            if let Some(c) = self.ch {
                if c.is_digit(10) {
                    digits.push(c);
                    continue;
                }
            }
            break;
        }

        Token::Lit(Lit::Int(LitInt { digits }))
    }

    fn read_str(&mut self) -> String {
        let is_letter = |c: char| c.is_ascii_alphanumeric() || c == '_';

        let ch = self
            .ch
            .expect("Entered string has already reached the end.");

        if !is_letter(ch) {
            panic!("A non-alphanumeric character was enterd.");
        }

        let mut literal = String::from(ch);
        loop {
            self.read_char();
            match self.ch {
                Some(ch) => {
                    if is_letter(ch) {
                        literal.push(ch);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        literal
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            Some(ch) => Some(match ch {
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '#' => Token::Sharp,
                ',' => Token::Comma,

                '0'..='9' => return Some(self.read_number()),

                _ => {
                    let token = match self.read_str().as_str() {
                        "LD" => Token::LD,
                        "ST" => Token::ST,
                        "ADD" => Token::ADD,

                        "r0" => Token::R0,
                        "r1" => Token::R1,
                        "r2" => Token::R2,
                        "r3" => Token::R3,
                        "r4" => Token::R4,
                        "r5" => Token::R5,
                        "r6" => Token::R6,
                        "r7" => Token::R7,

                        _ => unimplemented!(),
                    };

                    return Some(token);
                }
            }),
            None => None,
        };

        self.read_char();

        token
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = ((), Token, ());

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Some(token) => Some(((), token, ())),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_lexer {
        ($input: expr, $expected: expr) => {
            let mut lexer = Lexer::new($input);
            let mut tokens: Vec<Token> = Vec::new();

            while let Some(token) = lexer.next_token() {
                tokens.push(token);
            }

            assert_eq!($expected, tokens);
        };
    }

    macro_rules! token_int {
        ($value: expr) => {
            Token::Lit(Lit::Int(LitInt {
                digits: $value.to_string(),
            }))
        };
    }

    #[test]
    fn num() {
        test_lexer!("0", vec![token_int!(0)]);
        test_lexer!("1", vec![token_int!(1)]);
        test_lexer!("16", vec![token_int!(16)]);

        test_lexer!("#0", vec![Token::Sharp, token_int!(0)]);
        test_lexer!("#1", vec![Token::Sharp, token_int!(1)]);
        test_lexer!("#16", vec![Token::Sharp, token_int!(16)]);
    }

    #[test]
    fn symbol() {
        test_lexer!(",", vec![Token::Comma]);
        test_lexer!("#", vec![Token::Sharp]);
        test_lexer!("(", vec![Token::OpenParen]);
        test_lexer!(")", vec![Token::CloseParen]);
    }

    #[test]
    fn keyword_instruction() {
        test_lexer!("LD", vec![Token::LD]);
        test_lexer!("ST", vec![Token::ST]);
        test_lexer!("ADD", vec![Token::ADD]);
    }

    #[test]
    fn keyword_register() {
        test_lexer!("r0", vec![Token::R0]);
        test_lexer!("r1", vec![Token::R1]);
        test_lexer!("r2", vec![Token::R2]);
        test_lexer!("r3", vec![Token::R3]);
        test_lexer!("r4", vec![Token::R4]);
        test_lexer!("r5", vec![Token::R5]);
        test_lexer!("r6", vec![Token::R6]);
        test_lexer!("r7", vec![Token::R7]);
    }

    #[test]
    fn instruction() {
        test_lexer!(
            "LD r0, (r1)",
            vec![
                Token::LD,
                Token::R0,
                Token::Comma,
                Token::OpenParen,
                Token::R1,
                Token::CloseParen
            ]
        );

        test_lexer!(
            "ST r1, (r0)",
            vec![
                Token::ST,
                Token::R1,
                Token::Comma,
                Token::OpenParen,
                Token::R0,
                Token::CloseParen,
            ]
        );

        test_lexer!(
            "ADD r1, r2",
            vec![
                Token::ADD,
                Token::R1,
                Token::Comma,
                Token::R2,
            ]
        );
    }
}

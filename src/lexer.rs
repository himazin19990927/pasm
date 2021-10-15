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

        Token::Num(digits)
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
                '-' => Token::Minus,
                ':' => Token::Colon,
                ',' => Token::Comma,

                '0'..='9' => return Some(self.read_number()),

                _ => {
                    let token = match self.read_str().as_str() {
                        "LD" => Token::LD,
                        "LDI" => Token::LDI,
                        "ST" => Token::ST,
                        "ADD" => Token::ADD,
                        "ADDI" => Token::ADDI,

                        "r0" => Token::R0,
                        "r1" => Token::R1,
                        "r2" => Token::R2,
                        "r3" => Token::R3,
                        "r4" => Token::R4,
                        "r5" => Token::R5,
                        "r6" => Token::R6,
                        "r7" => Token::R7,

                        ident => Token::Ident(ident.to_string()),
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
    use std::vec;

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

    macro_rules! token_num {
        ($value: expr) => {
            Token::Num($value.to_string())
        };
    }

    #[test]
    fn num() {
        test_lexer!("0", vec![token_num!(0)]);
        test_lexer!("1", vec![token_num!(1)]);
        test_lexer!("16", vec![token_num!(16)]);

        test_lexer!("-1", vec![Token::Minus, token_num!(1)]);
        test_lexer!("-16", vec![Token::Minus, token_num!(16)]);

        test_lexer!("#0", vec![Token::Sharp, token_num!(0)]);
        test_lexer!("#1", vec![Token::Sharp, token_num!(1)]);
        test_lexer!("#16", vec![Token::Sharp, token_num!(16)]);

        test_lexer!("#-1", vec![Token::Sharp, Token::Minus, token_num!(1)]);
        test_lexer!("#-16", vec![Token::Sharp, Token::Minus, token_num!(16)]);
    }

    #[test]
    fn symbol() {
        test_lexer!(",", vec![Token::Comma]);
        test_lexer!("#", vec![Token::Sharp]);
        test_lexer!("-", vec![Token::Minus]);
        test_lexer!(":", vec![Token::Colon]);
        test_lexer!("(", vec![Token::OpenParen]);
        test_lexer!(")", vec![Token::CloseParen]);
    }

    #[test]
    fn ident() {
        test_lexer!("label", vec![Token::Ident("label".to_string())]);
        test_lexer!(
            ":label",
            vec![Token::Colon, Token::Ident("label".to_string())]
        );
        test_lexer!(
            ":label1",
            vec![Token::Colon, Token::Ident("label1".to_string())]
        );
    }

    #[test]
    fn keyword_instruction() {
        test_lexer!("LD", vec![Token::LD]);
        test_lexer!("LDI", vec![Token::LDI]);
        test_lexer!("ST", vec![Token::ST]);
        test_lexer!("ADD", vec![Token::ADD]);
        test_lexer!("ADDI", vec![Token::ADDI]);
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
            "LDI r0, #1",
            vec![
                Token::LDI,
                Token::R0,
                Token::Comma,
                Token::Sharp,
                token_num!(1),
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
            vec![Token::ADD, Token::R1, Token::Comma, Token::R2,]
        );

        test_lexer!(
            "ADDI r0, #1",
            vec![
                Token::ADDI,
                Token::R0,
                Token::Comma,
                Token::Sharp,
                token_num!(1),
            ]
        );
    }
}

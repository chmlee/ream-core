use std::iter::Peekable;
use std::str::Chars;
use crate::scanner::*;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {

    pub fn new(source: String) -> Result<Self, String> {
        let mut scanner = Scanner::new(source.chars());
        let tokens = scanner.scan()?.to_vec();
        Ok(
            Parser {
                tokens
            }
        )
    }
}

// impl Iterator for Parser {
//     type Item = Token;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.tokens.next()
//     }
// }

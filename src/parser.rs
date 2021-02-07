use crate::scanner::*;

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(content: String) -> Self {
        let mut scanner = Scanner::new(content.chars());
        let tokens = match scanner.scan() {
            Ok(tk) => tk,
            Err(e) => panic!("Something is wrong: {}", e),
        };
        let tokens = tokens
                       .to_vec();
        Parser {
            tokens
        }
    }



}

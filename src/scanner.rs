use std::collections::VecDeque;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub struct Token(TokenType);

// pub struct Marker {
//     line: usize,
//     col: usize,
// }

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Header(usize),
    Identifier(String),
    String(String),
    Colon,
    Dash,
    Space,
    NewLine,
}

#[derive(Debug)]
pub struct Scanner<T: Iterator<Item = char>>  {
    pub chars: Peekable<T>,
    pub tokens: Vec<Token>,

    ignore_spaces: bool,
}

impl<T: Iterator<Item = char>> Scanner<T> {

    pub fn new(source: T) -> Scanner<T> {
        let chars = source.peekable();
        Scanner {
            chars,
            tokens: Vec::new(),

            ignore_spaces: false,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.chars.next();
        c
    }

    pub fn peek(&mut self) -> Option<&char> {
        let c = self.chars.peek();
        c
    }

    pub fn push_token(&mut self, token_type: TokenType) {
        self.tokens.push(
            Token(token_type)
        );
    }

    pub fn scan(&mut self) {
        while let Some(&c) = self.peek() {
            match c {
                '#'  => self.scan_header(),
                ':'  => self.scan_colon(),
                '-'  => self.scan_dash(),
                ' '  => self.scan_spaces(),
                '\n' => self.scan_newlines(),
                '"'  => self.scan_value_string(),
                _    => self.scan_identifier(),
            }
        }
    }

    pub fn scan_header(&mut self) {
        let mut count = 0;
        while let Some(&c) = self.peek() {
            if c == '#' {
                count += 1;
                self.next();
            } else {
                break
            }
        }

        self.push_token(TokenType::Header(count))
    }

    pub fn scan_identifier(&mut self) {
        let mut string = String::new();
        while let Some(&c) = self.peek() {
            match c {
                ' ' | '\n' | ':' => break, // TODO: check for all utf8 whitespaces
                _ => {
                    string.push(c);
                    self.next();
                }
            }
        }

        self.push_token(TokenType::Identifier(string));
    }

    pub fn scan_colon(&mut self) {
        self.push_token(TokenType::Colon);
        self.next();
    }

    pub fn scan_dash(&mut self) {
        self.push_token(TokenType::Dash);
        self.next();
    }

    pub fn scan_spaces(&mut self) {
        while let Some(&c) = self.peek() {
            match c {
                ' ' => { self.next(); },
                _ => { break },
            }
        }
        self.push_token(TokenType::Space);
    }

    pub fn scan_newlines(&mut self) {
        while let Some(&c) = self.peek() {
            match c {
                '\n' => { self.next(); },
                _ => break,
            }
        }
        self.push_token(TokenType::NewLine);
    }

    pub fn scan_value_string(&mut self) {
        self.next(); // consume the opening `"`
        let mut ignore_quotationg = false;
        let mut string = String::new();
        while let Some(&c) = self.peek() {
            match c { // TODO: escpae quotation
                '"' => {
                    self.next();
                    break;
                },
                _ => {
                    string.push(c);
                    self.next();
                }
            }
        }

        self.push_token(TokenType::String(string))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn header_1() {
        let it = String::from("#");
        let mut scanner = Scanner::new(it.chars());
        scanner.scan_header();
        assert_eq!(
            scanner.tokens[0],
            Token{ token_type: TokenType::Header(1) }
        );
    }

    #[test]
    fn header_n() {
        let text = String::from("###");
        let mut scanner = Scanner::new(text.chars());
        scanner.scan_header();
        assert_eq!(
            scanner.tokens[0],
            Token{ token_type: TokenType::Header(3) }
        )
    }

    #[test]
    fn identifier() {
        let text = String::from("Name");
        let mut scanner = Scanner::new(text.chars());
        scanner.scan_identifier();
        assert_eq!(
            scanner.tokens[0],
            Token{ token_type: TokenType::Identifier(String::from("Name")) }
        )
    }

    #[test]
    fn identifier_no_space() {
        let text = String::from("Name other");
        let mut scanner = Scanner::new(text.chars());
        scanner.scan_identifier();
        assert_eq!(
            scanner.tokens[0],
            Token{ token_type: TokenType::Identifier(String::from("Name")) }
        )
    }


}

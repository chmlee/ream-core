use std::collections::VecDeque;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token(TokenType);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Marker {
    line: usize,
    col: usize,
}

impl Marker {
    fn new() -> Self {
        Marker {
            line: 1,
            col: 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Header(usize),
    Identifier(String),
    String(String),
    Number(String),
    Boolean(bool),
    Colon,
    Dash,
    WhiteSpace(usize),
    LineBreak(usize),

    Error,
}

#[derive(Debug)]
pub struct Scanner<T: Iterator<Item = char>>  {
    pub chars: Peekable<T>,
    pub tokens: Vec<Token>,

    pub marker: Marker,

}

macro_rules! scan_keyword {
    ($self:ident, $name:ident, $keyword:expr, $token_type:expr) => {
        pub fn $name(&mut $self) {
            for c in $keyword.chars() {
                if Some(&c) == $self.peek() {
                    $self.next();
                } else {
                    $self.push_token(TokenType::Error);
                    // return TokenType::Error;
                }
            }
            $self.push_token($token_type)
            // return $token_type;
        }
    }
}



impl<T: Iterator<Item = char>> Scanner<T> {


    pub fn new(source: T) -> Scanner<T> {
        let chars = source.peekable();
        Scanner {
            chars,
            tokens: Vec::new(),
            marker: Marker::new(),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.chars.next();
        self.marker.col += 1;
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

    pub fn scan(&mut self) -> Result<&Vec<Token>, String> {
        self.scan_token_whitespaces(0)?;
        while let Some(&c) = self.peek() {
            println!("{:?}", self.tokens);
            println!("scanning new line");
            match c {
                '#' => self.scan_line_header()?,
                '-' => self.scan_line_variable()?,
                _   => { return Err(String::from("wrong!")) },
            }
        }

        Ok(&self.tokens)
    }


    pub fn scan_token_whitespaces(&mut self, min: usize) -> Result<(), String> {
        let mut count = 0;
        while let Some(&c) = self.peek() {
            match c {
                ' ' => {
                    count += 1;
                    self.next();
                },
                _ => { break },
            }
        }

        if count < min {
            return Err(format!("Expecting at least {} spaces, found {}", min, count));
        }

        self.push_token(TokenType::WhiteSpace(count));

        Ok(())
    }

    pub fn scan_token_identifier(&mut self) -> Result<(), String> {
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

        println!("{}", string);
        if string.len() > 0 {
            self.push_token(TokenType::Identifier(string));
        } else {
            self.push_token(TokenType::Error);
        }

        Ok(())
    }

    pub fn scan_token_linebreaks(&mut self) -> Result<(), String> {
        let mut count = 0;
        while let Some(&c) = self.peek() {
            match c {
                '\n' => {
                    self.marker.line += 1;
                    count += 1;
                    self.next();
                },
                _ => break,
            }
        }
        self.push_token(TokenType::LineBreak(count));


        Ok(())
    }

    pub fn scan_token_header_level(&mut self) -> Result<(),String> {
        let mut count = 0;
        while let Some(&c) = self.peek() {
            if c == '#' {
                count += 1;
                self.next();
            } else {
                break
            }
        }
        self.push_token(TokenType::Header(count));

        Ok(())
    }

    pub fn scan_token_colon(&mut self) -> Result<(), String> {
        if let Some(':') = self.peek() {
            self.push_token(TokenType::Colon);
            self.next();
            return Ok(())
        }
        Err(String::from("Missing Colon"))
    }

    pub fn scan_token_dash(&mut self) -> Result<(), String> {
        if let Some('-') = self.peek() {
            self.push_token(TokenType::Dash);
            self.next();
            return Ok(())
        }
        Err(String::from("Missing Dash"))
    }

    pub fn scan_line_header(&mut self) -> Result<(),String> {
        self.scan_token_header_level()?;

        self.scan_token_whitespaces(1)?;

        self.scan_token_identifier()?;

        self.scan_token_whitespaces(0)?;
        self.scan_token_linebreaks()?;

        Ok(())
    }

    pub fn scan_line_variable(&mut self) -> Result<(), String> {
        self.scan_token_dash()?;

        self.scan_token_whitespaces(1)?;

        self.scan_token_identifier()?;

        self.scan_token_whitespaces(0)?;

        self.scan_token_colon()?;

        self.scan_token_string(0)?;

        self.scan_token_whitespaces(0)?;
        self.scan_token_linebreaks()?;


    }



    // scan_keyword!(self, scan_true, "true", TokenType::Boolean(true));
    // scan_keyword!(self, scan_false, "true", TokenType::Boolean(false));

    // pub fn scan_number(&mut self) { // TODO: support all possible number types
    //     let mut number = String::new();
    //     while let Some(&c) = self.peek() {
    //         match c {
    //             '0'..='9'  => {
    //                 number.push(c);
    //                 self.next();
    //             },
    //             _ => break,
    //         }
    //     }

    //     self.push_token(TokenType::Number(number));
    // }








    //     self.push_token(TokenType::String(string))
    // }


}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn header_1() {
        let it = String::from("#");
        let mut scanner = Scanner::new(it.chars());
        scanner.scan_token_header_level();
        assert_eq!(
            scanner.tokens[0],
            Token(TokenType::Header(1))
        );
    }

    #[test]
    fn header_n() {
        let text = String::from("###");
        let mut scanner = Scanner::new(text.chars());
        scanner.scan_token_header_level();
        assert_eq!(
            scanner.tokens[0],
            Token(TokenType::Header(3))
        )
    }

    #[test]
    fn identifier() {
        let text = String::from("Name");
        let mut scanner = Scanner::new(text.chars());
        scanner.scan_token_identifier();
        assert_eq!(
            scanner.tokens[0],
            Token(TokenType::Identifier(String::from("Name")))
        )
    }

    #[test]
    fn identifier_no_space() {
        let text = String::from("Name other");
        let mut scanner = Scanner::new(text.chars());
        scanner.scan_token_identifier();
        assert_eq!(
            scanner.tokens[0],
            Token(TokenType::Identifier(String::from("Name")))
        )
    }

    #[test]
    fn positive_integer() {
        let text = String::from("12848392");
        let mut scanner = Scanner::new(text.chars());
        scanner.scan_number();
        assert_eq!(
            scanner.tokens[0],
            Token(TokenType::Number(String::from("12848392")))
        )
    }

    // #[test]
    // fn tt() {
    //     let text = String::from("tue");
    //     let mut scanner = Scanner::new(text.chars());
    //     let result = scanner.scan_true();
    //     assert_eq!(
    //         result, TokenType::Error
    //     )
    // }

}

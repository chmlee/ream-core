// use std::collections::VecDeque;
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
    Star,
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

macro_rules! scan_token_keyword {
    ($self:ident, $name:ident, $keyword:expr, $token_type:expr, $err_msg:expr) => {
        pub fn $name(&mut $self) -> Result<(), String> {
            for c in $keyword.chars() { // TODO: check out of bound
                if Some(&c) == $self.peek() {
                    $self.next();
                } else {
                    return Err($err_msg)
                }
            }
            $self.push_token($token_type);

            Ok(())
        }
    }
}

macro_rules! scan_token_symbol {
    ($self:ident, $name:ident, $symbol:expr, $ok:expr, $error:expr) => {
        pub fn $name(&mut $self) -> Result<(), String> {
            if let Some($symbol) = $self.peek() {
                $self.push_token($ok);
                $self.next();
                return Ok(())
            }
            $error
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
            println!("{}", c);
            match c {
                '#' => self.scan_line_header()?,
                '-' => self.scan_line_variable()?,
                // '*' => self.scan_line_list_item()?,
                _   => { return Err(String::from("Invalid token")) },
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

    scan_token_symbol!(self, scan_token_colon, ':', TokenType::Colon, Err(String::from("Missing Colon")));

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


    scan_token_symbol!(self, scan_token_dash, '-', TokenType::Dash, Err(String::from("Missing Dash")));

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
        self.scan_token_whitespaces(0)?;
        self.scan_value()?;
        self.scan_token_whitespaces(0)?;
        self.scan_token_linebreaks()?;

        Ok(())
    }

    pub fn scan_value(&mut self) -> Result<(), String> {
        if let Some(&c) = self.peek() {
            match c {
                '"'       => self.scan_token_string()?,
                '0'..='9' => self.scan_token_number()?,
                't'       => self.scan_token_true()?,
                'f'       => self.scan_token_false()?,
                '\n'      => self.scan_list_items()?,
                _         => return Err(String::from("Invliad value")),
            }
        }

        Ok(())
    }

    scan_token_symbol!(self, scan_token_star, '*', TokenType::Star, Err(String::from("Missing Values!")));


    pub fn scan_list_items(&mut self) -> Result<(), String> {
        self.scan_token_linebreaks()?;
        self.scan_token_whitespaces(0)?;
        // println!("{:?}", self.peek());

        println!("{:?}", self.peek());
        while self.peek() == Some(&'*') {
            self.scan_token_star()?;

            self.scan_token_whitespaces(1)?;

            self.scan_value()?;

            self.scan_token_whitespaces(0)?;
            self.scan_token_linebreaks()?;
            self.scan_token_whitespaces(0)?;
        }

        Ok(())
    }

    pub fn scan_token_number(&mut self) -> Result<(), String> { // TODO: support all possible number types
        let mut number = String::new();
        while let Some(&c) = self.peek() {
            match c {
                '0'..='9'  => {
                    number.push(c);
                    self.next();
                },
                ' ' | '\n' => break, // TODO: match all utf8 whitespaces
                _ => return Err(String::from("Invalid number")),
            }
        }

        self.push_token(TokenType::Number(number));
        Ok(())
    }

    pub fn scan_token_string(&mut self) -> Result<(), String> {
        self.next(); // consume opening `"`
        let mut string = String::new();
        let mut escape = false;
        while let Some(&c) = self.peek() {
            match c {
                '"' => {
                    if escape {
                        string.push(c);
                        self.next();
                        escape = false;
                    } else {
                        self.next();
                        self.push_token(TokenType::String(string));
                        break;
                    }
                },
                '/' => {
                    escape = true;
                    self.next();
                },
                _   => {
                    string.push(c);
                    self.next();
                }
            }
        }

        Ok(())
    }

    scan_token_keyword!(self, scan_token_true, "true", TokenType::Boolean(true), String::from("invalid boolean"));
    scan_token_keyword!(self, scan_token_false, "false", TokenType::Boolean(false), String::from("invalid boolean"));









    //     self.push_token(TokenType::String(string))
    // }


}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! test_scanner {
        ($name:ident, $method:ident, $input:expr, $output:expr) => {
            #[test]
            fn $name() {
                let text = String::from($input);
                let mut scanner = Scanner::new(text.chars());
                let _x = scanner.$method();
                assert_eq!(
                    scanner.tokens,
                    $output
                )
            }
        }
    }

    test_scanner!(
        header_level,
        scan_token_header_level,
        "###",
        vec![
            Token(TokenType::Header(3))
        ]
    );

    test_scanner!(
        string,
        scan_token_string,
        "\"this is a long string value\"",
        vec![
            Token(TokenType::String(String::from("this is a long string value")))
        ]
    );

    test_scanner!(
        identifier,
        scan_token_identifier,
        "Name",
        vec![
            Token(TokenType::Identifier(String::from("Name")))
        ]
    );

    test_scanner!(
        identifier_no_space,
        scan_token_identifier,
        "Name you should not see this",
        vec![
            Token(TokenType::Identifier(String::from("Name")))
        ]
    );

    test_scanner!(
        number,
        scan_token_number,
        "1234567890",
        vec![
            Token(TokenType::Number(String::from("1234567890")))
        ]
    );

    test_scanner!(
        boolean_true,
        scan_token_true,
        "true",
        vec![
            Token(TokenType::Boolean(true))
        ]
    );

    test_scanner!(
        boolean_false,
        scan_token_false,
        "false",
        vec![
            Token(TokenType::Boolean(false))
        ]
    );

    test_scanner!(
        star,
        scan_token_star,
        "*",
        vec![
            Token(TokenType::Star)
        ]
    );

    test_scanner!(
        list_item,
        scan_line_list_item,
        "  * \"item\"",
        vec![
            Token(TokenType::WhiteSpace(2)),
            Token(TokenType::Star),
            Token(TokenType::WhiteSpace(1)),
            Token(TokenType::String(String::from("item"))),
            Token(TokenType::WhiteSpace(0)),
            Token(TokenType::LineBreak(0)),
        ]
    );

}

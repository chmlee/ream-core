use std::collections::VecDeque;
use std::{fmt, str};

#[derive(PartialEq, Eq, Clone)]
pub struct Token(pub TokenType, pub Marker, pub Marker);

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: ({}, {}) - ({}, {})",
               self.0 , self.1.line, self.1.col, self.2.line, self.2.col)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Marker {
    line: usize,
    col: usize,
}

impl Marker {
    pub fn new(line: usize, col: usize) -> Self {
        Marker {
            line,
            col,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {

    Header(usize),
    End(usize),

    Class(String),
    Key(String),

    String(String),
    Number(String),
    Boolean(bool),

    Colon,
    Dash,
    Star,

    // WhiteSpace(usize),
    // LineBreak(usize),

    // Error,
}

type ScanResult = Result<(), ()>;

#[derive(Debug, Clone)]
pub struct Scanner<'source>  {
    pub source: &'source [u8],
    pub buffer: VecDeque<Token>,

    pub eof: bool,

    pub loc: Marker,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let source = source.as_bytes();
        Scanner {
            source: source,
            buffer: VecDeque::new(),

            eof: false,

            loc: Marker::new(1, 0),
        }
    }

    pub fn next_col(&mut self, f: &str) {
        self.get_source();
        println!("{:?} after {}", self.get_loc(), f);
        self.loc.col += 1;
    }

    pub fn next_line(&mut self) {
        self.get_source();
        // println!("{:?}", self.get_loc());
        self.loc.col = 0;
        self.loc.line += 1;
    }

    pub fn get_source(&self) {
        println!("{:?}", str::from_utf8(self.source).unwrap());
    }

    pub fn get_loc(&self) -> Marker {
        self.loc
    }

    pub fn update_source(&mut self, rest: &'source [u8]) {
        self.source = rest;
        self.next_col("update");
    }

    pub fn push_token(&mut self, tt: TokenType) {
        let end = self.get_loc();

        let Marker{ line, col } = self.get_loc();
        let col = match &tt {
            TokenType::Dash
            | TokenType::Colon       => col,
            TokenType::Header(n)     => col - n + 1,
            TokenType::Class(s)
            | TokenType::Key(s)
            | TokenType::String(s)   => col - s.len() + 1,
            _ => col,
        };
        let start = Marker::new(line, col);

        self.buffer.push_back(Token(tt, start, end));
    }

    pub fn take_token(&mut self) -> Result<Option<Token>, ()> {
        if self.buffer.is_empty() {
            if self.eof {
                return Ok(None);   // End of File
            } else {
                self.scan_line()?; // add tokens to buffer
            }
        }

        let token = self.buffer.pop_front();
        // println!("{:?}", token);
        Ok(token)
    }

    pub fn skip_whitespaces(&mut self, min: usize) -> ScanResult {
        let mut count = 0;
        loop {
            match self.source {
                [b' ', ref rest @ ..] => { // TODO: add all utf8 whitespaces
                    self.update_source(rest);
                    count += 1;
                },
                _ => break,
            }
        }

        if count < min {
            panic!("too few spaces!");
        }

        Ok(())
    }

    pub fn scan_line(&mut self) -> ScanResult {

        // ignore all empty lines
        loop {
            match self.source {
                [b'\n', ref rest @ ..] => {
                    self.update_source(rest);
                    self.next_line();
                },
                [] => {
                    self.eof = true;
                    return Ok(());
                }
                _ => break,
            }
        }

        self.skip_whitespaces(0)?;

        let token = match self.source {
            [token, ref rest @ ..] => {
                self.update_source(rest);
                token
            },
            [] => {
                self.eof = true;
                return Ok(());
            },
        };


        match token {
            b'#' => self.scan_line_header()?,
            b'-' => self.scan_line_variable()?,
            _ => panic!("Invalid token!"),
        }

        self.end_of_line()?;

        Ok(())
    }

    pub fn scan_line_variable(&mut self) -> ScanResult {
        self.push_token(TokenType::Dash);
        self.skip_whitespaces(1)?;
        self.scan_token_key()?;
        self.skip_whitespaces(0)?;
        self.scan_token_colon()?;
        self.skip_whitespaces(0)?;
        self.scan_value()?;

        Ok(())
    }

    pub fn scan_value(&mut self) -> ScanResult {
        let mut value = String::new();
        loop {
            match self.source {
                [b'\n', ref _rest @ ..] => {
                    break;
                },
                [b, ref rest @ ..] => {
                    value.push(*b as char);
                    self.update_source(rest);
                },
                _ => break, // TODO: ?
            }
        }

        if value.is_empty() {
            panic!("value is empty");
        }

        self.push_token(TokenType::String(value));

        Ok(())
    }

    pub fn scan_token_colon(&mut self) -> ScanResult {
        match self.source {
            [b':', ref rest @ ..] => {
                self.update_source(rest);
            },
            _ => panic!("expecting colon"),
        }

        self.push_token(TokenType::Colon);

        Ok(())
    }

    pub fn scan_line_header(&mut self) -> ScanResult {
        self.scan_token_header()?;
        self.skip_whitespaces(1)?;
        self.scan_token_class()?;

        Ok(())
    }

    pub fn scan_token_header(&mut self) -> ScanResult {
        let mut count = 1;
        loop {
            match self.source {
                [b'#', ref rest @ ..] => {
                    count += 1;
                    self.update_source(rest);
                },
                [b' ',  ..] => break,
                [b'\n', ..] => break,
                _ => panic!("expecting header!"),
            }
        }
        self.push_token(TokenType::Header(count));

        Ok(())
    }

    pub fn scan_token_key(&mut self) -> ScanResult {
        let mut name = String::new();
        loop {
            match self.source {
                [b':', ref _rest @ ..] => {
                    break;
                },
                [b' ', ref _rest @ ..] => {
                    break;
                },
                [b, ref rest @ ..] => {
                    name.push(*b as char);
                    self.update_source(rest);
                },
                _ => panic!("expecting key"),
            }
        }
        self.push_token(TokenType::Key(name));

        Ok(())
    }

    pub fn scan_token_class(&mut self) -> ScanResult {
        let mut name = String::new();
        loop {
            match self.source {
                [b'\n', ref _rest @ ..] => {
                    break;
                },
                [b, ref rest @ ..] => {
                    name.push(*b as char);
                    self.update_source(rest);
                },
                _ => break,
            }
        }

        if name.is_empty() {
            panic!("class is empty");
        }

        self.push_token(TokenType::Class(name));

        Ok(())
    }

    pub fn end_of_line(&mut self) -> ScanResult {
        self.skip_whitespaces(0)?;
        match self.source {

            [b'\n', ref rest @ ..] => {
                self.next_line();
                self.source = rest
            },
            [] => {
                println!("end of file");
                self.eof = true;
            },
            _ => {
                panic!("should end line!");
            },
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn header_line() {
        //          1234567
        let text = "# Title";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Header(1),
                        Marker{ line: 1, col: 1 },
                        Marker{ line: 1, col: 1 }
                ),
                Token(
                    TokenType::Class("Title".to_string()),
                        Marker{ line: 1, col: 3 },
                        Marker{ line: 1, col: 7 }
                ),
            ]
        )
    }

    #[test]
    fn varible_line_string() {
        //          0        1
        //          123456789012
        let text = "- key: value";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Dash,
                        Marker{ line: 1, col: 1 },
                        Marker{ line: 1, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                        Marker{ line: 1, col: 3 },
                        Marker{ line: 1, col: 5 },
                ),
                Token(
                    TokenType::Colon,
                        Marker{ line: 1, col: 6 },
                        Marker{ line: 1, col: 6 },
                ),
                Token(
                    TokenType::String("value".to_string()),
                        Marker{ line: 1, col: 8 },
                        Marker{ line: 1, col: 12 },
                ),
            ]
        )
    }

    #[test]
    fn count_spaces() {
        //          0        1
        //          12345678901234
        let text = " - key : value";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Dash,
                        Marker{ line: 1, col: 2 },
                        Marker{ line: 1, col: 2 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                        Marker{ line: 1, col: 4 },
                        Marker{ line: 1, col: 6 },
                ),
                Token(
                    TokenType::Colon,
                        Marker{ line: 1, col: 8 },
                        Marker{ line: 1, col: 8 },
                ),
                Token(
                    TokenType::String("value".to_string()),
                        Marker{ line: 1, col: 10 },
                        Marker{ line: 1, col: 14 },
                ),
            ]
        )
    }

    #[test]
    fn multiple_lines() {
        let text = "# Title\n- key: value";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Header(1),
                        Marker{ line: 1, col: 1 },
                        Marker{ line: 1, col: 1 }
                ),
                Token(
                    TokenType::Class("Title".to_string()),
                        Marker{ line: 1, col: 3 },
                        Marker{ line: 1, col: 7 }
                ),
                Token(
                    TokenType::Dash,
                        Marker{ line: 2, col: 1 },
                        Marker{ line: 2, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                        Marker{ line: 2, col: 3 },
                        Marker{ line: 2, col: 5 },
                ),
                Token(
                    TokenType::Colon,
                        Marker{ line: 2, col: 6 },
                        Marker{ line: 2, col: 6 },
                ),
                Token(
                    TokenType::String("value".to_string()),
                        Marker{ line: 2, col: 8 },
                        Marker{ line: 2, col: 12 },
                ),
            ]
        )
    }

    #[test]
    fn skip_empty_lines() {
        let text = "# Title\n\n- key: value";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Header(1),
                        Marker{ line: 1, col: 1 },
                        Marker{ line: 1, col: 1 }
                ),
                Token(
                    TokenType::Class("Title".to_string()),
                        Marker{ line: 1, col: 3 },
                        Marker{ line: 1, col: 7 }
                ),
                Token(
                    TokenType::Dash,
                        Marker{ line: 3, col: 1 },
                        Marker{ line: 3, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                        Marker{ line: 3, col: 3 },
                        Marker{ line: 3, col: 5 },
                ),
                Token(
                    TokenType::Colon,
                        Marker{ line: 3, col: 6 },
                        Marker{ line: 3, col: 6 },
                ),
                Token(
                    TokenType::String("value".to_string()),
                        Marker{ line: 3, col: 8 },
                        Marker{ line: 3, col: 12 },
                ),
            ]
        )
    }
}

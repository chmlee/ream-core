use std::iter::Peekable;
use std::collections::VecDeque;
use std::{fmt, str};

#[derive(PartialEq, Eq, Clone)]
pub struct Token(pub TokenType);

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{:?}",
               self.0 /*, self.1.line, self.1.col*/)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Marker {
    pub line: usize,
    pub col: usize,
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
    source: &'source [u8],
    buffer: VecDeque<Token>,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let source = source.as_bytes();
        Scanner {
            source: source,
            buffer: VecDeque::new(),
        }
    }

    pub fn get_source(&self) {
        println!("{:?}", str::from_utf8(self.source).unwrap());
    }

    pub fn push_token(&mut self, token: Token) {
        self.buffer.push_back(token);
    }

    pub fn skip_whitespaces(&mut self, min: usize) -> ScanResult {
        let mut count = 0;
        loop {
            match self.source {
                [b' ', ref rest @ ..] => {
                    self.source = rest;
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

        self.skip_whitespaces(0)?;

        let (token, rest) = match self.source {
            [tk, ref rest @ ..] => (tk, rest),
            _ => panic!("panic!"),
        };

        self.source = rest;

        match token {
            b'#' => self.scan_line_header()?,
            b'-' => self.scan_line_variable()?,
            _ => panic!("Invalid token!"),
        }

        self.end_of_line()?;

        Ok(())
    }

    pub fn scan_line_variable(&mut self) -> ScanResult {
        self.skip_whitespaces(1)?;
        self.scan_token_key()?;
        self.skip_whitespaces(0)?;
        self.scan_token_colon()?;
        self.skip_whitespaces(0)?;
        self.scan_value()?;

        Ok(())
    }

    pub fn scan_value(&mut self) -> ScanResult {
        self.get_source();
        let mut value = String::new();
        loop {
            match self.source {
                [b'\n', ref rest @ ..] => {
                    self.source = rest;
                    break;
                },
                [b, ref rest @ ..] => {
                    value.push(*b as char);
                    self.source = rest;
                },
                _ => panic!("unreachable"),
            }
        }

        self.push_token(Token(TokenType::String(value)));

        Ok(())
    }

    pub fn scan_token_colon(&mut self) -> ScanResult {
        match self.source {
            [b':', ref rest @ ..] => self.source = rest,
            _ => panic!("expecting colon"),
        }

        self.push_token(Token(TokenType::Colon));

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
                    self.source = rest;
                    count += 1;
                },
                [b' ', ..] => break,
                _ => panic!("panic!"),
            }
        }
        self.push_token(Token(TokenType::Header(count)));

        Ok(())
    }

    pub fn scan_token_key(&mut self) -> ScanResult {
        let mut name = String::new();
        loop {
            match self.source {
                [b':', ref _rest @ ..] => {
                    break;
                },
                [b, ref rest @ ..] => {
                    name.push(*b as char);
                    self.source = rest;
                },
                _ => panic!("panic!"),
            }
        }
        self.push_token(Token(TokenType::Key(name)));

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
                    self.source = rest;
                },
                _ => panic!("panic!"),
            }
        }
        self.push_token(Token(TokenType::Class(name)));

        Ok(())
    }

    pub fn end_of_line(&mut self) -> ScanResult {
        self.skip_whitespaces(0)?;
        match self.source {

            [b'\n', ref rest @ ..] => self.source = rest,
            [] => println!("end of file"),
            _ => panic!("should end line!"),
        }

        Ok(())
    }
}

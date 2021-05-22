use crate::error::*;
use crate::ream::*;

use std::collections::VecDeque;
use std::{fmt, str};


#[derive(PartialEq, Eq, Clone)]
pub struct Token(pub TokenType, pub Marker, pub Marker);

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: ({}, {}) - ({}, {})",
            self.0, self.1.line, self.1.col, self.2.line, self.2.col
        )
    }
}

impl Token {
    pub fn new(tt: TokenType, p0: Marker, p1: Marker) -> Self {
        Self(tt, p0, p1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Marker {
    line: usize,
    col: usize,
}

impl Marker {
    pub fn new(line: usize, col: usize) -> Self {
        Marker { line, col }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Header(usize),

    Class(String),
    Key(String),

    Value(String),
    ValueType(ValueType),

    Block(usize),
    Annotation(String),

    Colon,
    Dash,
    Star,
}

#[derive(Debug, Clone)]
pub struct Scanner<'source> {
    pub source: &'source [u8],
    pub buffer: VecDeque<Token>,

    pub eof: bool,
    pub loc: Marker,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let source = source.as_bytes();
        Scanner {
            source,
            buffer: VecDeque::new(),

            eof: false,

            loc: Marker::new(1, 0),
        }
    }

    pub fn next_col(&mut self) {
        self.loc.col += 1;
    }

    pub fn next_line(&mut self) {
        // self.get_source();
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
        self.next_col();
    }

    pub fn push_token(&mut self, tt: TokenType) {
        let end = self.get_loc();

        let Marker { line, col } = self.get_loc();
        let col = match &tt {
            TokenType::Dash | TokenType::Colon => col,
            TokenType::Header(n) => col - n + 1,
            TokenType::Class(s)
            | TokenType::Key(s)
            | TokenType::Value(s)
            | TokenType::Annotation(s) => col - s.len() + 1,
            TokenType::ValueType(t) => col - t.size() - 1,
            _ => col,
        };
        let start = Marker::new(line, col);

        self.buffer.push_back(Token(tt, start, end));
    }

    pub fn peek_token(&mut self) -> Result<Option<&Token>, ReamError> {
        if self.buffer.is_empty() {
            if self.eof {
                return Ok(None); // End of File
            } else {
                self.scan_line()?; // add tokens to buffer
            }
        }

        let token_option = self.buffer.front();
        Ok(token_option)
    }

    pub fn take_token(&mut self) -> Result<Option<Token>, ReamError> {
        if self.buffer.is_empty() {
            if self.eof {
                return Ok(None); // End of File
            } else {
                self.scan_line()?; // add tokens to buffer
            }
        }

        let token_option = self.buffer.pop_front();
        Ok(token_option)
    }

    pub fn skip_whitespaces(&mut self, min: usize) -> Result<(), ReamError> {
        let mut count = 0;
        loop {
            match self.source {
                [b' ', ref rest @ ..] => {
                    // TODO: add all utf8 whitespaces
                    self.update_source(rest);
                    count += 1;
                }
                _ => break,
            }
        }

        if count < min {
            return Err(ReamError::ScanError(ScanErrorType::WrongHeaderLevel));
        }

        Ok(())
    }

    pub fn scan_line(&mut self) -> Result<(), ReamError> {
        // ignore all empty lines
        loop {
            match self.source {
                [b'\n', ref rest @ ..] => {
                    self.update_source(rest);
                    self.next_line();
                }
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
            }
            [] => {
                self.eof = true;
                return Ok(());
            }
        };

        match token {
            b'#' => self.scan_line_header()?,
            b'-' => self.scan_line_variable()?,
            b'>' => self.scan_line_annotation()?,
            b'*' => self.scan_line_list_item()?,
            _ => return Err(ReamError::ScanError(ScanErrorType::InvalidToken)),
        }

        self.end_of_line()?;

        Ok(())
    }

    pub fn scan_line_list_item(&mut self) -> Result<(), ReamError> {
        self.push_token(TokenType::Star);
        self.skip_whitespaces(1)?;
        self.scan_value()?;

        Ok(())
    }

    pub fn scan_token_block(&mut self) -> Result<(), ReamError> {
        let mut count = 1;
        loop {
            match self.source {
                [b'>', ref rest @ ..] => {
                    count += 1;
                    self.update_source(rest);
                }
                [b' ', ..] => break,
                [b'\n', ..] => break,
                _ => return Err(ReamError::ScanError(ScanErrorType::InvalidToken)),
            }
        }
        self.push_token(TokenType::Block(count));

        Ok(())
    }

    pub fn scan_token_annotation(&mut self) -> Result<(), ReamError> {
        let mut ann = String::new();
        loop {
            match self.source {
                [b'\n', ref _rest @ ..] => {
                    break;
                }
                [b, ref rest @ ..] => {
                    ann.push(*b as char);
                    self.update_source(rest);
                }
                _ => break, // TODO: ?
            }
        }

        self.push_token(TokenType::Annotation(ann));

        Ok(())
    }

    pub fn scan_line_annotation(&mut self) -> Result<(), ReamError> {
        self.scan_token_block()?;
        self.skip_whitespaces(1)?;
        self.scan_token_annotation()?;

        Ok(())
    }

    pub fn scan_line_header(&mut self) -> Result<(), ReamError> {
        self.scan_token_header()?;
        self.skip_whitespaces(1)?;
        self.scan_token_class()?;

        Ok(())
    }

    pub fn scan_line_variable(&mut self) -> Result<(), ReamError> {
        // - key (type): value
        self.push_token(TokenType::Dash);
        self.skip_whitespaces(1)?;
        self.scan_token_key()?;
        self.skip_whitespaces(0)?;
        self.scan_token_value_type()?;
        self.skip_whitespaces(0)?;
        self.scan_token_colon()?;
        self.skip_whitespaces(0)?;
        self.scan_value()?;

        Ok(())
    }

    pub fn parse_unit_type(&self, t: &str) -> Result<ValueType, ReamError> {
        let typ = match t {
            "str" => ValueType::Str,
            "num" => ValueType::Num,
            "bool" => ValueType::Bool,
            "list" => ValueType::List(Box::new(ValueType::Unknown)),
            "ref" => ValueType::Ref(Box::new(ValueType::Unknown)),
            _ => return Err(ReamError::TypeError(TypeErrorType::UnknownType)),
        };

        Ok(typ)
    }

    pub fn fold_types(&self, acc: ValueType, new_typ: ValueType) -> Result<ValueType, ReamError> {
        let next = match acc {
            ValueType::Unknown => {
                new_typ
            },
            ValueType::List(_) => {
                ValueType::List(Box::new(new_typ))
            },
            ValueType::Ref(t) => {
                match *t {
                    ValueType::List(_) => ValueType::Ref(Box::new(ValueType::List(Box::new(new_typ)))),
                    _ => ValueType::Ref(Box::new(new_typ)),

                }
            },
            _ => {
                return Err(ReamError::TypeError(TypeErrorType::UnknownType));
            },
        };

        Ok(next)
    }

    pub fn scan_value_type_inner(&mut self) -> Result<ValueType, ReamError> {
        let mut result = ValueType::Unknown;
        let mut new_type_str = String::new();
        loop {
            match self.source {
                [b')', rest @ ..] => {
                    self.update_source(rest);
                    result = self.fold_types(result, self.parse_unit_type(new_type_str.as_str())?)?;
                    return Ok(result);
                },
                [b' ', rest @ ..] => {
                    self.update_source(rest);
                    result = self.fold_types(result, self.parse_unit_type(new_type_str.as_str())?)?;
                    new_type_str = String::new();
                },
                [b'\n', _rest @ ..] => {
                    return Err(ReamError::TypeError(TypeErrorType::UnknownType));
                },
                [b, rest @ ..] => {
                    self.update_source(rest);
                    new_type_str.push(*b as char);
                },
                _ => unreachable!(),
            }
        };
    }

    pub fn scan_token_value_type(&mut self) -> Result<(), ReamError> {
        let typ: ValueType = match self.source {
            [b'(', rest @ ..] => {
                self.update_source(rest);
                self.scan_value_type_inner()?
            }
            [_, _rest @ ..] => {
                ValueType::Unknown
            }
            _ => return Err(ReamError::ScanError(ScanErrorType::InvalidToken)),
        };

        // only known type will be pushed to buffer
        match typ {
            ValueType::Unknown => {},
            t => self.push_token(TokenType::ValueType(t)),
        }

        Ok(())
    }

    pub fn scan_value(&mut self) -> Result<(), ReamError> {
        let mut value = String::new();
        loop {
            match self.source {
                [b'\n', ref _rest @ ..] => {
                    break;
                }
                [b, ref rest @ ..] => {
                    value.push(*b as char);
                    self.update_source(rest);
                }
                _ => break, // TODO: ?
            }
        }

        if value.is_empty() {
            return Ok(());
        }

        self.push_token(TokenType::Value(value));

        Ok(())
    }

    pub fn scan_token_colon(&mut self) -> Result<(), ReamError> {
        match self.source {
            [b':', ref rest @ ..] => {
                self.update_source(rest);
            }
            _ => return Err(ReamError::ScanError(ScanErrorType::MissingColon)),
        }

        self.push_token(TokenType::Colon);

        Ok(())
    }

    pub fn scan_token_header(&mut self) -> Result<(), ReamError> {
        let mut count = 1;
        loop {
            match self.source {
                [b'#', ref rest @ ..] => {
                    count += 1;
                    self.update_source(rest);
                }
                [b' ', ..] => break,
                [b'\n', ..] => break,
                // TODO: other?
                _ => return Err(ReamError::ScanError(ScanErrorType::InvalidToken)),
            }
        }
        self.push_token(TokenType::Header(count));

        Ok(())
    }

    pub fn scan_token_key(&mut self) -> Result<(), ReamError> {
        let mut name = String::new();
        loop {
            match self.source {
                [b':', ref _rest @ ..] => {
                    break;
                }
                [b' ', ref _rest @ ..] => {
                    break;
                }
                [b, ref rest @ ..] => {
                    name.push(*b as char);
                    self.update_source(rest);
                }
                _ => return Err(ReamError::ScanError(ScanErrorType::MissingKey)),
            }
        }
        self.push_token(TokenType::Key(name));

        Ok(())
    }

    pub fn scan_token_class(&mut self) -> Result<(), ReamError> {
        let mut name = String::new();
        loop {
            match self.source {
                [b'\n', ref _rest @ ..] => {
                    break;
                }
                [b, ref rest @ ..] => {
                    name.push(*b as char);
                    self.update_source(rest);
                }
                _ => break,
            }
        }

        if name.is_empty() {
            return Err(ReamError::ScanError(ScanErrorType::MissingClass));
        }

        self.push_token(TokenType::Class(name));

        Ok(())
    }

    pub fn end_of_line(&mut self) -> Result<(), ReamError> {
        self.skip_whitespaces(0)?;
        match self.source {
            [b'\n', ref rest @ ..] => {
                self.next_line();
                self.source = rest
            }
            [] => {
                println!("end of file");
                self.eof = true;
            }
            _ => {
                return Err(ReamError::ScanError(ScanErrorType::MissingEOL));
            }
        }
        // println!("end of line!");

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
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 }
                ),
                Token(
                    TokenType::Class("Title".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 7 }
                ),
            ]
        )
    }

    #[test]
    fn variable_line_string() {
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
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 5 },
                ),
                Token(
                    TokenType::Colon,
                    Marker { line: 1, col: 6 },
                    Marker { line: 1, col: 6 },
                ),
                Token(
                    TokenType::Value("value".to_string()),
                    Marker { line: 1, col: 8 },
                    Marker { line: 1, col: 12 },
                ),
            ]
        )
    }

    #[test]
    fn varible_line_string_with_type() {
        //          0        1
        //          1234567890123456789
        let text = "- key (str): value";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Dash,
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 5 },
                ),
                Token(
                    TokenType::ValueType(ValueType::Unit(UnitType::Str)),
                    Marker { line: 1, col: 7 },
                    Marker { line: 1, col: 11 },
                ),
                Token(
                    TokenType::Colon,
                    Marker { line: 1, col: 12 },
                    Marker { line: 1, col: 12 },
                ),
                Token(
                    TokenType::Value("value".to_string()),
                    Marker { line: 1, col: 14 },
                    Marker { line: 1, col: 18 },
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
                    Marker { line: 1, col: 2 },
                    Marker { line: 1, col: 2 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                    Marker { line: 1, col: 4 },
                    Marker { line: 1, col: 6 },
                ),
                Token(
                    TokenType::Colon,
                    Marker { line: 1, col: 8 },
                    Marker { line: 1, col: 8 },
                ),
                Token(
                    TokenType::Value("value".to_string()),
                    Marker { line: 1, col: 10 },
                    Marker { line: 1, col: 14 },
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
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 }
                ),
                Token(
                    TokenType::Class("Title".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 7 }
                ),
                Token(
                    TokenType::Dash,
                    Marker { line: 2, col: 1 },
                    Marker { line: 2, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                    Marker { line: 2, col: 3 },
                    Marker { line: 2, col: 5 },
                ),
                Token(
                    TokenType::Colon,
                    Marker { line: 2, col: 6 },
                    Marker { line: 2, col: 6 },
                ),
                Token(
                    TokenType::Value("value".to_string()),
                    Marker { line: 2, col: 8 },
                    Marker { line: 2, col: 12 },
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
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 }
                ),
                Token(
                    TokenType::Class("Title".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 7 }
                ),
                Token(
                    TokenType::Dash,
                    Marker { line: 3, col: 1 },
                    Marker { line: 3, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                    Marker { line: 3, col: 3 },
                    Marker { line: 3, col: 5 },
                ),
                Token(
                    TokenType::Colon,
                    Marker { line: 3, col: 6 },
                    Marker { line: 3, col: 6 },
                ),
                Token(
                    TokenType::Value("value".to_string()),
                    Marker { line: 3, col: 8 },
                    Marker { line: 3, col: 12 },
                ),
            ]
        )
    }

    #[test]
    fn annotation_line() {
        //          0        1
        //          1234567890123
        let text = "> hello world";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Block(1),
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 }
                ),
                Token(
                    TokenType::Annotation("hello world".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 13 }
                ),
            ]
        )
    }

    #[test]
    fn variable_with_annotation() {
        //          0        1    0        1
        //          1234567890123 12345678901234567
        let text = "- key: value\n> some annotation";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Dash,
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 },
                ),
                Token(
                    TokenType::Key("key".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 5 },
                ),
                Token(
                    TokenType::Colon,
                    Marker { line: 1, col: 6 },
                    Marker { line: 1, col: 6 },
                ),
                Token(
                    TokenType::Value("value".to_string()),
                    Marker { line: 1, col: 8 },
                    Marker { line: 1, col: 12 },
                ),
                Token(
                    TokenType::Block(1),
                    Marker { line: 2, col: 1 },
                    Marker { line: 2, col: 1 }
                ),
                Token(
                    TokenType::Annotation("some annotation".to_string()),
                    Marker { line: 2, col: 3 },
                    Marker { line: 2, col: 17 }
                ),
            ]
        )
    }

    #[test]
    fn list_item() {
        //          0
        //          12345678
        let text = "* item";
        let mut scanner = Scanner::new(&text);
        let _ = scanner.scan_line();
        let _ = scanner.scan_line();
        assert_eq!(
            scanner.buffer,
            vec![
                Token(
                    TokenType::Star,
                    Marker { line: 1, col: 1 },
                    Marker { line: 1, col: 1 },
                ),
                Token(
                    TokenType::Value("item".to_string()),
                    Marker { line: 1, col: 3 },
                    Marker { line: 1, col: 6 },
                ),
            ]
        )
    }
}

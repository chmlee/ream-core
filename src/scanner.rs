use std::iter::Peekable;
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub struct Token(TokenType, Marker);

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: ({}, {})\n",
               self.0, self.1.line, self.1.col)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Marker {
    line: usize,
    col: usize,
}

impl Marker {
    fn new(line: usize, col: usize) -> Self {
        Marker {
            line,
            col,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Identifier(String),

    String(String),
    Number(String),
    Boolean(bool),

    Header(usize),
    End(usize),

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
    pub allow_list: bool,
    pub list_has_item: bool,
    pub level: usize,

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
            marker: Marker::new(1, 1),

            allow_list: false,
            list_has_item: false,
            level: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.chars.next();
        // println!("{:?}: {:?}", &c, self.marker);
        self.marker.col += 1;
        c
    }

    pub fn peek(&mut self) -> Option<&char> {
        let c = self.chars.peek();
        c
    }

    pub fn push_token(&mut self, token_type: TokenType) {
        let col = match &token_type {
            TokenType::Boolean(true)   => self.marker.col - 4,
            TokenType::Boolean(false)  => self.marker.col - 5,
            TokenType::Header(n)
            | TokenType::WhiteSpace(n) => self.marker.col - n,
            TokenType::Colon
            | TokenType::Dash
            | TokenType::LineBreak(_)
            | TokenType::Star          => self.marker.col,
            TokenType::String(s)       => self.marker.col - s.len() - 2, // add quotation marks
            | TokenType::Number(s)
            | TokenType::Identifier(s) => self.marker.col - s.len(),
            TokenType::Error
            | TokenType::End(_)        => self.marker.col,
        };
        let marker_start = Marker::new(self.marker.line, col);
        // let marker_end = Marker::new(self.marker.line, self.marker.col - 1);
        self.tokens.push(
            Token(token_type, marker_start)
        );
    }

    pub fn scan(&mut self) -> Result<&Vec<Token>, String> {
        self.scan_token_whitespaces(0)?;
        while let Some(&c) = self.peek() {
            match c {
                '#' => self.scan_line_header()?,
                '-' => self.scan_line_variable()?,
                '*' => self.scan_line_list_item()?,
                _   => return Err(String::from("Invalid token")),
            }
        }

        for level in (1..=self.level).rev() {
            println!("{}", level);
            self.push_token(TokenType::End(level));
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

        // check minimum number of whitespaces
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
                    count += 1;
                    self.next();
                    self.marker.line += 1;
                    self.marker.col = 1;
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

        println!("{}, {}", self.level, count);
        // Add End tokens if not child
        if count <= self.level {
            println!("a");
            for level in (count..=self.level).rev() {
                println!("{}", level);
                self.push_token(TokenType::End(level));
            }
        }
        self.level = count;

        self.push_token(TokenType::Header(count));

        Ok(())
    }


    scan_token_symbol!(self, scan_token_dash, '-', TokenType::Dash, Err(String::from("Missing Dash")));

    pub fn scan_line_header(&mut self) -> Result<(),String> {
        if self.allow_list && !self.list_has_item {
            return Err(String::from("Missing value"));
        }
        if self.allow_list {
            self.allow_list = false;
        }

        self.scan_token_header_level()?;
        self.scan_token_whitespaces(1)?;
        self.scan_token_identifier()?;
        self.scan_token_whitespaces(0)?;
        self.scan_token_linebreaks()?;
        self.scan_token_whitespaces(0)?;

        Ok(())
    }

    pub fn scan_line_variable(&mut self) -> Result<(), String> {
        if self.allow_list && !self.list_has_item {
            return Err(String::from("Missing value"));
        }
        if self.allow_list {
            self.allow_list = false;
        }

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

    pub fn scan_line_list_item(&mut self) -> Result<(), String> {
        if !self.allow_list {
            return Err(String::from("Unexpected Star"));
        }

        self.scan_token_star()?;

        self.scan_token_whitespaces(1)?;

        self.scan_value()?;

        self.scan_token_whitespaces(0)?;
        self.scan_token_linebreaks()?;
        self.scan_token_whitespaces(0)?;

        self.list_has_item = true;
        Ok(())
    }

    pub fn scan_value(&mut self) -> Result<(), String> {
        if let Some(&c) = self.peek() {
            match c {
                '"'       => self.scan_token_string()?,
                '0'..='9' => self.scan_token_number()?,
                't'       => self.scan_token_true()?,
                'f'       => self.scan_token_false()?,
                '\n'      => {
                    self.allow_list = true;
                    self.scan_token_linebreaks()?;
                },
                _         => return Err(String::from("Invliad value")),
            }
        }

        Ok(())
    }

    scan_token_symbol!(self, scan_token_star, '*', TokenType::Star, Err(String::from("Missing Values!")));



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
            Token(TokenType::Header(3), Marker{line:1, col:1})
        ]
    );

    test_scanner!(
        string,
        scan_token_string,
        "\"this is a long string value\"",
        vec![
            Token(TokenType::String(String::from("this is a long string value")), Marker{line:1, col:1})
        ]
    );

    test_scanner!(
        identifier,
        scan_token_identifier,
        "Name",
        vec![
            Token(TokenType::Identifier(String::from("Name")), Marker{line:1, col:1})
        ]
    );

    test_scanner!(
        identifier_no_space,
        scan_token_identifier,
        "Name you should not see this",
        vec![
            Token(TokenType::Identifier(String::from("Name")), Marker{line:1, col:1})
        ]
    );

    test_scanner!(
        number,
        scan_token_number,
        "1234567890",
        vec![
            Token(TokenType::Number(String::from("1234567890")), Marker{line:1, col:1})
        ]
    );

    test_scanner!(
        boolean_true,
        scan_token_true,
        "true",
        vec![
            Token(TokenType::Boolean(true), Marker{line:1, col:1})
        ]
    );

    test_scanner!(
        boolean_false,
        scan_token_false,
        "false",
        vec![
            Token(TokenType::Boolean(false), Marker{line:1, col:1})
        ]
    );

    test_scanner!(
        star,
        scan_token_star,
        "*",
        vec![
            Token(TokenType::Star, Marker{line:1, col:1})
        ]
    );

}

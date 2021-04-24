use crate::scanner::*;
use crate::ream::*;

#[derive(Debug)]
pub struct Parser<'source> {
    pub scanner: Scanner<'source>,
    pub level: usize,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(source),
            level: 0,
        }
    }

    pub fn parse_token_header(&mut self) -> Result<usize, ScanError> {
        let level = match self.scanner.take_token()? {
            Some(Token(TokenType::Header(n), _, _)) => n,
            _ => panic!("expecting header level"),
        };

        Ok(level)
    }

    pub fn parse_token_identifier(&mut self) -> Result<String, ScanError> {
        let identifier = match self.scanner.take_token()? {
            Some(Token(TokenType::Class(c), _, _))
            | Some(Token(TokenType::Key(c), _, _))   => c,
            _ => panic!("expecting identifier"),
        };

        Ok(identifier)
    }

    pub fn parse_entry(&mut self) -> ParseEntryResult {
        let level = self.parse_token_header()?;
        println!("parsing entry level {}", &level);
        self.level = level;

        let class = self.parse_token_identifier()?;

        let mut entry = Entry::new(class, level);

        // loop for variables
        while let Some(Token(TokenType::Dash, _, _)) = self.scanner.peek_token()? {
            self.scanner.take_token()?;
            // println!("{:?}", self.scanner.buffer);
            let result = self.parse_variable()?;
            match result {
                Some(var) => entry.push_variable(var),
                None => panic!("expecting variables"),
            }
        }

        // println!("{:?}", self.scanner.buffer);

        // loop for subentries
        let mut subentries = vec![];
        while let Some(Token(TokenType::Header(next_level), _, _)) = self.scanner.peek_token()? {
            if *next_level == self.level + 1 {               // child
                let subentry = match self.parse_entry()? {
                    Some(sub) => sub,
                    None => panic!("expecting subentry"),
                };
                subentries.push(subentry);
            } else if *next_level <= self.level {
                self.level -= 1;
                break;
            } else {
                panic!("wrong level of entry");
            }
        }
        entry.subentries = subentries;

        Ok(Some(entry))
    }

    pub fn parse_variable(&mut self) -> ParseVariableResult {

        let key = self.parse_token_identifier()?;


        let mut typ = match self.scanner.take_token()? {
            Some(Token(TokenType::Type(t), _, _)) => {
                self.parse_symbol_colon()?;
                let t = match t.as_str() {
                    "str" => ValueType::Str,
                    "num" => ValueType::Num,
                    "bool" => ValueType::Bool,
                    _ => panic!("unidentified type"),
                };
                t
            },
            Some(Token(TokenType::Colon, _, _)) => {
                ValueType::Unknown
            },
            _ => {
                panic!("unexpected token");
            }
        };

        let val = match self.scanner.take_token()? {
            Some(Token(TokenType::Value(v), _, _)) => v,
            _ => panic!("expecting value"),
        };

        typ = match typ {
            ValueType::Unknown => {
                check_unknown_value_type(&val)?
            },
            t => {
                validate_known_value_type(&val, &t)?;
                t
            }
        };

        Ok(Some(Variable::new(key, typ, val)))
    }

    pub fn parse_symbol_colon(&mut self) -> Result<(),ScanError> {
        match self.scanner.take_token()? {
            Some(Token(TokenType::Colon, _, _)) => Ok(()),
            _ => panic!("expecting Colon"),
        }
    }


}

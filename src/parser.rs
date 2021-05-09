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

    pub fn parse_token_header(&mut self) -> Result<usize, ReamError> {
        let level = match self.scanner.take_token()? {
            Some(Token(TokenType::Header(n), _, _)) => n,
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingHeaderLevel)),
        };

        Ok(level)
    }

    pub fn parse_token_identifier(&mut self) -> Result<String, ReamError> {
        let identifier = match self.scanner.take_token()? {
            Some(Token(TokenType::Class(c), _, _))
            | Some(Token(TokenType::Key(c), _, _))   => c,
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingIdentifier)),
        };

        Ok(identifier)
    }

    pub fn parse_entry(&mut self) -> Result<Option<Entry>, ReamError> {
        let level = self.parse_token_header()?;
        // println!("parsing entry level {}", &level);
        self.level = level;

        let class = self.parse_token_identifier()?;

        let mut entry = Entry::new(class, level);

        // loop for variables
        while let Some(Token(TokenType::Dash, _, _)) = self.scanner.peek_token()? {
            self.scanner.take_token()?;
            // println!("{:?}", self.scanner.buffer);
            let result = self.parse_variable()?;
            match result {
                Some(var) => {
                    entry.push_variable(var);
                },
                None => {
                    return Err(ReamError::ParseError(ParseErrorType::MissingVariable));
                },
            }
        }

        // println!("{:?}", self.scanner.buffer);

        // loop for subentries
        while let Some(Token(TokenType::Header(next_level), _, _)) = self.scanner.peek_token()? {
            if next_level.to_owned() == self.level + 1 {               // child
                let subentry = match self.parse_entry()? {
                    Some(sub) => sub,
                    None => return Err(ReamError::ParseError(ParseErrorType::MissingSubentry)),
                };
                entry.push_subentry(subentry);
            } else if next_level.to_owned() <= self.level {
                self.level -= 1;
                break;
            } else {
                return Err(ReamError::ParseError(ParseErrorType::WrongHeaderLevel));
            }
        }

        Ok(Some(entry))
    }

    pub fn parse_variable(&mut self) -> Result<Option<Variable>, ReamError> {

        let key = self.parse_token_identifier()?;


        let typ = match self.scanner.take_token()? {
            Some(Token(TokenType::Type(t), _, _)) => {
                self.parse_symbol_colon()?;
                let t = match t.as_str() {
                    "str" => ValueType::Str,
                    "num" => ValueType::Num,
                    "bool" => ValueType::Bool,
                    _ => return Err(ReamError::TypeError(TypeErrorType::UnknownType)),
                };
                t
            },
            Some(Token(TokenType::Colon, _, _)) => {
                ValueType::Unknown
            },
            _ => {
                return Err(ReamError::ParseError(ParseErrorType::MissingToken));
            }
        };

        let val = match self.scanner.take_token()? {
            Some(Token(TokenType::Value(v), _, _)) => {
                match typ {
                    ValueType::Unknown => {
                        check_unknown_value_type(v)?
                    },
                    t => {
                        validate_known_value_type(v, t)?
                    },
                }

            },
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingValue)),
        };

        let ann = match self.scanner.peek_token()? {
            Some(Token(TokenType::Block(_), _, _)) => {
                self.scanner.take_token()?; // consume Block
                match self.scanner.take_token()? {
                    Some(Token(TokenType::Annotation(s), _, _)) => s,
                    _ => unreachable!(),
                }
            },
            _ => {
                String::from("")
            }
        };

        Ok(Some(Variable::new(key, val, ann)))
    }

    pub fn parse_symbol_colon(&mut self) -> Result<(), ReamError> {
        match self.scanner.take_token()? {
            Some(Token(TokenType::Colon, _, _)) => Ok(()),
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingColon)),
        }
    }


}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn header_line() {
        let text = "# Title";
        let mut parser = Parser::new(&text);
        let entry_test = parser.parse_entry().unwrap().unwrap();
        let entry_ans = Entry::new("Title".to_string(), 1);
        println!("{:?}", entry_test);
        assert_eq!(entry_test, entry_ans);
    }

    #[test]
    fn varible_line_string() {
        let text = "# Title\n- key: value\n> annotation";
        let mut parser = Parser::new(&text);
        let entry_test = parser.parse_entry().unwrap().unwrap();
        let mut entry_ans = Entry::new("Title".to_string(), 1);
        let var = Variable::new(
            String::from("key"),
            ReamValue::Str("value".to_string()),
            String::from("annotation"),
        );
        entry_ans.push_variable(var);
        assert_eq!(entry_test, entry_ans);
    }

}

use crate::ream::*;
use crate::scanner::*;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct Parser<'source> {
    pub scanner: Scanner<'source>,
    pub current_level: usize,
    pub current_class: String,
    pub history: BTreeMap<String, HashMap<String, ReamValue>>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(source),
            current_level: 0,
            current_class: String::new(),
            history: BTreeMap::new(),
        }
    }

    pub fn get_current_class(&self) -> String {
        self.current_class.clone()
    }

    pub fn parse_header(&mut self) -> Result<usize, ReamError> {
        let level = match self.scanner.take_token()? {
            Some(Token(TokenType::Header(n), _, _)) => n,
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingHeaderLevel)),
        };

        Ok(level)
    }

    pub fn parse_identifier(&mut self) -> Result<String, ReamError> {
        let identifier = match self.scanner.take_token()? {
            Some(Token(TokenType::Class(c), _, _)) | Some(Token(TokenType::Key(c), _, _)) => c,
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingIdentifier)),
        };

        Ok(identifier)
    }

    pub fn parse_entry(&mut self) -> Result<Option<Entry>, ReamError> {
        let level = self.parse_header()?;
        // println!("parsing entry level {}", &level);
        self.current_level = level;

        let class = self.parse_identifier()?;
        self.current_class = class.clone();

        let mut entry = Entry::new(class, level);

        // loop for variables
        while let Some(Token(TokenType::Dash, _, _)) = self.scanner.peek_token()? {
            self.scanner.take_token()?;
            // println!("{:?}", self.scanner.buffer);
            let result = self.parse_variable()?;
            match result {
                Some(var) => {
                    entry.push_variable(var);
                }
                None => {
                    return Err(ReamError::ParseError(ParseErrorType::MissingVariable));
                }
            }
        }

        // println!("{:?}", self.scanner.buffer);

        // loop for subentries
        while let Some(Token(TokenType::Header(next_level), _, _)) = self.scanner.peek_token()? {
            if next_level.to_owned() == self.current_level + 1 {
                // child
                let subentry = match self.parse_entry()? {
                    Some(sub) => sub,
                    None => return Err(ReamError::ParseError(ParseErrorType::MissingSubentry)),
                };
                entry.push_subentry(subentry);
            } else if next_level.to_owned() <= self.current_level {
                self.current_level -= 1;
                break;
            } else {
                return Err(ReamError::ParseError(ParseErrorType::WrongHeaderLevel));
            }
        }

        Ok(Some(entry))
    }

    pub fn parse_variable(&mut self) -> Result<Option<ReamVariable>, ReamError> {
        let key = self.parse_identifier()?;

        let typ = self.parse_type()?;
        // println!("{:?}", typ);

        self.parse_colon()?;

        let val = self.parse_value(typ)?;

        let ann = self.parse_annotation()?;

        self.add_ref(key.clone(), val.clone());
        let val_ann = ReamValueAnnotated::new(val, ann);

        Ok(Some(ReamVariable::new(key, val_ann)))
    }

    // TODO: use lifetime
    pub fn add_ref(&mut self, key: String, val: ReamValue) {

        if !self.history.contains_key(&self.current_class) {
            let mut new_inner: HashMap<String, ReamValue> = HashMap::new();
            new_inner.insert(key, val);
            self.history.insert(self.current_class.clone(), new_inner);
        } else {
            let inner = self.history.get_mut(&self.current_class).unwrap();
            inner.insert(key, val);
        }
    }

    pub fn get_ref(&mut self, tok: Option<Token>, typ: ValueType) -> Result<ReamValue, ReamError> {
        let (s, c, r) = match tok {
            Some(Token(TokenType::Value(s), c, r)) => {
                (s, c, r)
            },
            _ => unreachable!(),
        };

        // split class and key names by `$`
        let v: Vec<&str> = s
            .split('$')
            .collect();

        if let [class, key] = &v[..] {
            match self.history.get(*class) {
                None => {
                    return Err(ReamError::ReferenceError(ReferenceErrorType::EntryClassNotFound));
                },
                Some(inner) => {
                    match inner.get(*key) {
                        None => {
                            return Err(ReamError::ReferenceError(ReferenceErrorType::VariableKeyNotFound));
                        },
                        Some(s) => {
                            println!("{:?}, {:?}", s, typ);
                            s.is_variant(typ)?;
                            Ok((*s).clone())
                        },
                    }
                }
            }
        } else {
           return Err(ReamError::ReferenceError(ReferenceErrorType::InvalidReference));
        }
    }

    pub fn parse_value(&mut self, typ: ValueType) -> Result<ReamValue, ReamError> {
        let val = self.scanner.take_token()?;
        match typ {
            ValueType::Ref(t) => {
                self.get_ref(val, *t)
            },
            _ => {
                match val {
                    Some(Token(TokenType::Value(v), _, _)) => ReamValue::new(v, typ),
                    Some(Token(TokenType::Star, _, _)) => self.parse_list_items(typ),
                    _ => return Err(ReamError::ParseError(ParseErrorType::MissingValue)),
                }
            }
        }
    }

    pub fn parse_list_items(&mut self, typ: ValueType) -> Result<ReamValue, ReamError> {
        // unwrap list type
        let typ = match typ {
            ValueType::List(t) => *t,
            ValueType::Unknown => ValueType::Unknown,
            _ => return Err(ReamError::TypeError(TypeErrorType::UnknownType)),
        };

        // parse first item
        let first_val = self.parse_value(typ.clone())?;
        let ann = self.parse_annotation()?;
        let val_ann = ReamValueAnnotated::new(first_val.clone(), ann);
        let mut list = vec![val_ann.clone()];

        // loop through list items
        loop {
            match self.scanner.peek_token()? {
                Some(Token(TokenType::Star, _, _)) => {
                    self.scanner.take_token()?; // consume star

                    let new_val = self.parse_value(typ.clone())?;
                    first_val.match_variant(&new_val)?; // check homogeneity
                    let ann = self.parse_annotation()?;
                    let new_val_ann = ReamValueAnnotated::new(new_val, ann);
                    list.push(new_val_ann);
                }
                _ => break,
            }
        }

        Ok(ReamValue::List(list))
    }

    pub fn parse_annotation(&mut self) -> Result<String, ReamError> {
        match self.scanner.peek_token()? {
            Some(Token(TokenType::Block(_), _, _)) => {
                self.scanner.take_token()?; // consume Block
                match self.scanner.take_token()? {
                    Some(Token(TokenType::Annotation(s), _, _)) => Ok(s),
                    _ => unreachable!(),
                }
            }
            _ => Ok(String::from("")),
        }
    }

    pub fn parse_type(&mut self) -> Result<ValueType, ReamError> {
        let typ = match self.scanner.peek_token()? {
            // value type is specified
            Some(Token(TokenType::ValueType(_), _, _)) => {
                let t = match self.scanner.take_token()? {
                    Some(Token(TokenType::ValueType(t), _, _)) => t,
                    _ => unreachable!(),
                };
                t
            }
            // value type not specified
            Some(Token(TokenType::Colon, _, _)) => ValueType::Unknown,
            // maybe unreachable?
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingColon)),
        };


        Ok(typ)
    }

    pub fn parse_colon(&mut self) -> Result<(), ReamError> {
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

    // #[test]
    // fn variable_line_string() {
    //     let text = "# Title\n- key: value\n> annotation";
    //     let mut parser = Parser::new(&text);
    //     let entry_test = parser.parse_entry().unwrap().unwrap();
    //     let mut entry_ans = Entry::new("Title".to_string(), 1);
    //     let var = ReamVariable::new(
    //         String::from("key"),
    //         ReamValue::Str("value".to_string()),
    //         String::from("annotation"),
    //     );
    //     entry_ans.push_variable(var);
    //     assert_eq!(entry_test, entry_ans);
    // }
}

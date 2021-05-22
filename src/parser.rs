use crate::ream::*;
use crate::scanner::*;
use crate::error::*;

use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug)]
pub struct Parser<'source> {
    pub scanner: Scanner<'source>,
    pub current_level: usize,
    pub class_history: Vec<String>,
    pub schemas: HashMap<String, EntrySchema>,
    // pub history: BTreeMap<String, HashMap<String, ReamValue>>,
}

#[derive(Debug, Clone)]
pub struct EntrySchema {
    keys: Vec<String>,
}

impl EntrySchema {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            keys,
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.keys.clone()
    }
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(source),
            current_level: 0,
            class_history: Vec::new(),
            schemas: HashMap::new(),
            // history: BTreeMap::new(),
        }
    }

    pub fn schema(&self, class: String) -> Result<EntrySchema, ReamError> {
        match self.schemas.get(&class) {
            Some(v) => Ok((*v).clone()), // TODO: clone!
            None => Err(ReamError::SchemaError(SchemaErrorType::IncorrectClass)),
        }
    }

    pub fn parent_class(&self) -> Option<String> {
        let level = self.current_level;
        match level {
            1 => None, // ignore root node
            _ => Some(self.class_history[level - 2].clone()), // TODO: clone!
        }
    }

    pub fn push_class(&mut self, new_class: String) {
        self.class_history.push(new_class);
    }

    pub fn pop_class(&mut self) {
        self.class_history.pop();
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

        // find entry level
        let level = self.parse_header()?;
        self.current_level = level;

        // find entry class
        let class = self.parse_identifier()?;
        self.push_class(class.clone()); // TODO: clone!

        // init entry
        let mut entry = Entry::new(class, level);

        // init history
        // let new_inner: HashMap<String, ReamValue> = HashMap::new();
        // self.history.insert(self.current_class.clone(), new_inner);
        // println!("init history:");
        // println!("{:#?}", &self.history);

        // loop for variables
        while let Some(Token(TokenType::Dash, _, _)) = self.scanner.peek_token()? {
            self.scanner.take_token()?; // consume Dash
            let (key, val) = self.parse_variable()?;
            entry.push_key(key.clone());
            entry.insert_variable(key, val)?;
        }

        // check schema
        let mut entry = self.check_schema(entry)?;

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

        // cleanup
        self.pop_class(); // pop current class

        Ok(Some(entry))
    }

    pub fn check_schema(&mut self, entry: Entry) -> Result<Entry, ReamError> {
        // if schema is not yet defined, init
        if self.schemas.contains_key(entry.class()) {
            self.check_schema_inner(entry)
        } else {
            self.init_schema(entry)
        }
    }

    pub fn init_schema(&mut self, entry: Entry) -> Result<Entry, ReamError> {
        let entry_keys = entry.keys();
        let entry_class = entry.class().clone();
        let entry_schema = EntrySchema::new(entry_keys);
        self.schemas.insert(entry_class, entry_schema); // TODO: clone!

        Ok(entry)
    }

    pub fn check_schema_inner(&self, entry: Entry) -> Result<Entry, ReamError> {
        let entry_keys = entry.keys();
        let entry_class = entry.class().clone();
        let schema = self.schema(entry_class)?;
        let schema_keys = schema.keys();

        if schema_keys == entry_keys {
            Ok(entry)
        } else {
            Err(ReamError::SchemaError(SchemaErrorType::IncorrectKeys))
        }
    }

    pub fn parse_variable(&mut self) -> Result<(String, Value), ReamError> {

        let key   = self.parse_identifier()?;
        let typ   = self.parse_type()?;
                    self.parse_colon()?;
        let value = self.parse_value(typ)?;

        Ok((key, value))
    }

    // TODO: use lifetime
    // pub fn add_ref(&mut self, key: String, val: ReamValue) -> Result<(), ReamError> {
    //     let inner = self.history.get_mut(&self.current_class).unwrap();

    //     // check duplicate keys
    //     match inner.insert(key, val) {
    //         None => Ok(()),
    //         Some(_) => Err(ReamError::ReferenceError(ReferenceErrorType::DuplicateKeys)),
    //     }
    // }

    // pub fn get_ref(&mut self, tok: Option<Token>, typ: ValueType) -> Result<ReamValue, ReamError> {
    //     let (s, c, r) = match tok {
    //         Some(Token(TokenType::Value(s), c, r)) => {
    //             (s, c, r)
    //         },
    //         _ => unreachable!(),
    //     };

    //     // split class and key names by `$`
    //     let v: Vec<&str> = s
    //         .split('$')
    //         .collect();

    //     if let [class, key] = &v[..] {
    //         match self.history.get(*class) {
    //             None => {
    //                 return Err(ReamError::ReferenceError(ReferenceErrorType::EntryClassNotFound));
    //             },
    //             Some(inner) => {
    //                 match inner.get(*key) {
    //                     None => {
    //                         return Err(ReamError::ReferenceError(ReferenceErrorType::VariableKeyNotFound));
    //                     },
    //                     Some(s) => {
    //                         // println!("{:?}, {:?}", s, typ);
    //                         s.is_variant(typ)?; // TODO: type checking does not work
    //                         Ok((*s).clone())
    //                     },
    //                 }
    //             }
    //         }
    //     } else {
    //        return Err(ReamError::ReferenceError(ReferenceErrorType::InvalidReference));
    //     }
    // }

    pub fn parse_value(&mut self, typ: ValueType) -> Result<Value, ReamError> {
        let tok_value = self.scanner.take_token()?;
        let (value_base, typ) = match typ {
            _ => {
                match tok_value {
                    Some(Token(TokenType::Value(v), _, _)) => ValueBase::new(v, typ)?,
                    Some(Token(TokenType::Star, _, _)) => self.parse_list_items(typ)?,
                    _ => return Err(ReamError::ParseError(ParseErrorType::MissingValue)),
                }
            }
        };

        let annotation = self.parse_annotation()?;

        let value = Value::new(value_base, annotation, typ);

        Ok(value)
    }

    pub fn parse_list_items(&mut self, typ: ValueType) -> Result<(ValueBase, ValueType), ReamError> {

        // unwrap list type
        let typ = match typ {
            ValueType::List(t) => *t,
            ValueType::Unknown => ValueType::Unknown,
            _ => return Err(ReamError::TypeError(TypeErrorType::UnknownType)),
        };

        // parse first item
        let first_item = self.parse_value(typ.clone())?;

        // init list
        let item_typ = first_item.typ().clone(); // get the updated type
        let mut list = List::new(item_typ.clone(), first_item);

        // loop through list items
        loop {
            match self.scanner.peek_token()? {
                Some(Token(TokenType::Star, _, _)) => {
                    self.scanner.take_token()?; // consume star
                    let new_item = self.parse_value(typ.clone())?;
                    // check new item type
                    if new_item.typ() == list.item_type() {
                        list.push_item(new_item);
                    } else {
                        return Err(ReamError::TypeError(TypeErrorType::HeterogeneousList))
                    }
                }
                _ => break,
            }
        }

        let value_base = ValueBase::new_item(list);
        let typ = ValueType::List(Box::new(item_typ.clone()));
        Ok((value_base, typ))
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

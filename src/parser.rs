use crate::error::*;
use crate::format::*;
use crate::scanner::*;
use crate::decorator::Decorator;

use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug)]
pub struct Parser<'source> {
    pub scanner: Scanner<'source>,
    pub current_level: usize,
    pub class_history: Vec<String>,
    pub schemas: HashMap<String, EntrySchema>,

    upstream: HashMap<String, VariableMap>,
    downstream: HashMap<String, Vec<VariableMap>>,
    parse_direction: Direction,
    ref_keys_buffer: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Down,
    Up,
} 

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(source),
            current_level: 0,
            class_history: vec!["_root_".to_string()],
            schemas: HashMap::new(),

            upstream: HashMap::new(),
            downstream: HashMap::new(),
            parse_direction: Direction::Down,
            ref_keys_buffer: Vec::new(),
        }
    }

    pub fn push_ref_key(&mut self, key: String) {
        self.ref_keys_buffer.push(key);
    }

    pub fn get_schema(&self, class: String) -> Result<EntrySchema, ReamError> {
        match self.schemas.get(&class) {
            Some(v) => Ok((*v).clone()), // TODO: clone!
            None => Err(ReamError::SchemaError(SchemaErrorType::IncorrectSchema)),
        }
    }

    pub fn parent_class(&self) -> Option<String> {
        let level = self.current_level;
        match level {
            1 => None,                                        // root node
            _ => Some(self.class_history[level - 2].clone()), // TODO: clone!
        }
    }

    pub fn push_class(&mut self, new_class: String) {
        self.class_history.push(new_class);
    }

    pub fn pop_class(&mut self) {
        self.class_history.pop();
    }

    // pub fn current_class(&self) -> &String {
    //     match self.class_history.last() {
    //         Some(c) => c,
    //         None => unreachable!(),
    //     }
    // }

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

    pub fn parse_decorators(&mut self) -> Result<Option<Vec<Decorator>>, ReamError> {
        let mut decorators: Vec<Decorator> = vec![];

        // loop through decorators
        while let Some(Token(TokenType::At(_), _, _)) = self.scanner.peek_token()? {
            self.scanner.take_token()?; // consume At
            let decorator = self.parse_decorator()?;
            decorators.push(decorator)
        }

        if decorators.is_empty() {
            Ok(None)
        } else {
            Ok(Some(decorators))
        }
    }

    pub fn parse_decorator(&mut self) -> Result<Decorator, ReamError> {
        match self.scanner.take_token()? {
            Some(Token(TokenType::Decorator(d), _, _)) => Decorator::from(d.clone()), // TODO: clone!
            _ => Err(ReamError::DecoratorError(DecoratorErrorType::InvalidDecorator)),
        }
    }



    pub fn parse_entry(&mut self) -> Result<Option<Entry>, ReamError> {

        // find decorators
        let decorators = self.parse_decorators()?;

        // find entry level
        let level = self.parse_header()?;
        self.current_level = level;

        // find entry class
        let class = self.parse_identifier()?;
        self.push_class(class.clone()); // TODO: clone!

        // find parent class
        let parent_class = self.parent_class();

        // init entry
        let mut entry = Entry::new(class, level, parent_class, decorators);

        // loop for variables
        while let Some(Token(TokenType::Dash, _, _)) = self.scanner.peek_token()? {
            self.scanner.take_token()?; // consume Dash
            let (key, val) = self.parse_variable()?;
            entry.push_key(key.clone());
            entry.insert_variable(key, val)?;
        }

        // check schema
        let mut entry = self.check_schema(entry)?;

        // update upstream
        self.upstream.insert(entry.class(), entry.variable_map());

        // move unresolved ref keys from parser to entry
        entry.set_ref_key(self.ref_keys_buffer.clone()); // TODO: clone!
        self.ref_keys_buffer = Vec::new();

        // loop for subentries
        while let Some(Token(TokenType::Header(next_level), _, _))
            | Some(Token(TokenType::At(next_level), _, _)) = self.scanner.peek_token()? {
            if next_level.to_owned() == self.current_level + 1 {
                // child entry
                self.parse_direction = Direction::Down;
                let subentry = match self.parse_entry()? {
                    Some(sub) => sub,
                    None => return Err(ReamError::ParseError(ParseErrorType::MissingSubentry)),
                };
                // let subentry = self.parse_entry()?;
                entry.push_subentry(subentry);
            } else if next_level.to_owned() <= self.current_level {
                // return to parent entry
                self.parse_direction = Direction::Up;
                self.current_level -= 1;
                break;
            } else {
                // wrong level for subentry
                return Err(ReamError::ParseError(ParseErrorType::WrongHeaderLevel));
            }
        }

        // downstream reference
        entry.resolve_downstream_ref(&self.downstream)?;

        // cleanup

        // pop current class
        self.pop_class();

        // move current entry from upstream to downstream
        let variable_map = match self.upstream.get(&entry.class()) {
            Some(map) => map.clone(),
            None => return Err(ReamError::Placeholder),
        };
        self.upstream.remove(&entry.class());
        self.insert_downstream(entry.class().clone(), variable_map.clone());


        Ok(Some(entry))
    }

    pub fn insert_downstream(&mut self, class: String, variable_map: VariableMap) {
        if let Some(x) = self.downstream.get_mut(&class) {
            x.push(variable_map);
        } else {
            self.downstream.insert(class.clone(), vec![variable_map]);
        }
    }

    pub fn check_schema(&mut self, entry: Entry) -> Result<Entry, ReamError> {
        if self.schemas.contains_key(&entry.class()) {
            // schema exist -> check
            self.check_schema_inner(entry)
        } else {
            // schema does not exist -> init
            self.init_schema(entry)
        }
    }

    pub fn init_schema(&mut self, entry: Entry) -> Result<Entry, ReamError> {
        let entry_keys = entry.keys();
        let entry_parent_class = entry.get_parent_class();
        let entry_schema = EntrySchema::new(entry_keys, entry_parent_class);

        let entry_class = entry.class().clone(); // TODO: clone!
        self.schemas.insert(entry_class, entry_schema);

        Ok(entry)
    }

    pub fn check_schema_inner(&self, entry: Entry) -> Result<Entry, ReamError> {
        let entry_schema = entry.get_schema();
        let parser_schema = self.get_schema(entry.class())?;

        if entry_schema == parser_schema {
            Ok(entry)
        } else {
            Err(ReamError::SchemaError(SchemaErrorType::IncorrectKeys))
        }
    }

    pub fn parse_variable(&mut self) -> Result<(String, Value), ReamError> {
        let key = self.parse_identifier()?;
        let typ = self.parse_type()?;
        self.parse_colon()?;
        let value = self.parse_value(&key, typ)?;

        Ok((key, value))
    }

    pub fn parse_value(&mut self, key: &String, typ: ValueType) -> Result<Value, ReamError> {
        let tok_value = self.scanner.take_token()?;
        let (value_base, typ) = match tok_value {
            Some(Token(TokenType::Value(v), _, _)) => {
                match typ {
                    // if value is a reference, get the reference
                    ValueType::Ref => {
                        let (value_base, typ) = self.get_ref(v)?;
                        match value_base {
                            // unresolved reference will be pushed to ref_key_buffer
                            // and checked for downstream reference
                            ValueBase::Ref(c, k) => {
                                self.push_ref_key(key.clone());
                                (ValueBase::new_ref(c, k), typ)
                            },
                            _ => (value_base, typ),
                        }
                    },
                    _ => ValueBase::new(v, typ)?,
                }
            }
            Some(Token(TokenType::Star, _, _)) => self.parse_list_items(&key, typ)?,
            _ => return Err(ReamError::ParseError(ParseErrorType::MissingValue)),
        };

        let annotation = self.parse_annotation()?;

        let value = Value::new(value_base, annotation, typ);

        Ok(value)
    }

    pub fn get_ref(&self, value: String) -> Result<(ValueBase, ValueType), ReamError> {
        let v: Vec<&str> = value.split('$').collect();

        if let [class, key] = &v[..] {
            match self.upstream.get(*class) {
                Some(variable_map) => match variable_map.get(&key.to_string()) {
                    Some(s) => Ok(s.get_base_and_typ()),
                    None => Err(ReamError::ReferenceError(
                        ReferenceErrorType::VariableKeyNotFound,
                    )),
                },
                None => match self.parse_direction {
                    Direction::Down => Ok((
                            ValueBase::new_ref(
                                (*class).to_string(),
                                (*key).to_string(),
                            ),
                            ValueType::Ref,
                    )),
                    Direction::Up => Err(ReamError::ReferenceError(
                        ReferenceErrorType::EntryClassNotFound,
                    )),
                }
            }
        } else {
            return Err(ReamError::ReferenceError(
                ReferenceErrorType::InvalidReference,
            ));
        }
    }

    pub fn parse_list_items(
        &mut self,
        key: &String,
        typ: ValueType,
    ) -> Result<(ValueBase, ValueType), ReamError> {
        // unwrap list type
        let typ = match typ {
            ValueType::List(t) => *t,
            ValueType::Unknown => ValueType::Unknown,
            _ => return Err(ReamError::TypeError(TypeErrorType::UnknownType)),
        };

        // parse first item
        let first_item = self.parse_value(&key, typ.clone())?;

        // init list
        let item_typ = first_item.typ().clone(); // get the updated type
        let mut list = List::new(item_typ.clone(), first_item);

        // loop through list items
        loop {
            match self.scanner.peek_token()? {
                Some(Token(TokenType::Star, _, _)) => {
                    self.scanner.take_token()?; // consume star
                    let new_item = self.parse_value(&key, typ.clone())?;
                    // check new item type
                    if new_item.typ() == list.item_type() {
                        list.push_item(new_item);
                    } else {
                        return Err(ReamError::TypeError(TypeErrorType::HeterogeneousList));
                    }
                }
                _ => break,
            }
        }

        let value_base = ValueBase::new_item(list);
        let typ = ValueType::List(Box::new(item_typ.clone()));
        Ok((value_base, typ))
    }

    pub fn parse_annotation(&mut self) -> Result<Option<String>, ReamError> {
        match self.scanner.peek_token()? {
            Some(Token(TokenType::Block(_), _, _)) => {
                self.scanner.take_token()?; // consume Block
                match self.scanner.take_token()? {
                    Some(Token(TokenType::Annotation(s), _, _)) => Ok(Some(s)),
                    _ => return Err(ReamError::Placeholder),
                }
            }
            _ => Ok(None),
        }
    }

    pub fn parse_type(&mut self) -> Result<ValueType, ReamError> {
        let typ = match self.scanner.peek_token()? {
            // value type is specified
            Some(Token(TokenType::ValueType(_), _, _)) => {
                let t = match self.scanner.take_token()? {
                    Some(Token(TokenType::ValueType(t), _, _)) => t,
                    _ => return Err(ReamError::Placeholder),
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

    // use super::*;

    // #[test]
    // fn header_line() {
    //     let text = "# Title";
    //     let mut parser = Parser::new(&text);
    //     let entry_test = parser.parse_entry().unwrap().unwrap();
    //     let entry_ans = Entry::new("Title".to_string(), 1);
    //     assert_eq!(entry_test, entry_ans);
    // }

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

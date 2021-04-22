use serde::{Serialize, Deserialize};
use crate::scanner::ScanError;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub class: String,
    pub level: usize,

    pub variables: Vec<Variable>,
    pub subentries: Vec<Entry>,
}

impl Entry {
    pub fn new(class: String, level: usize) -> Self {
        Entry {
            class,
            level,

            variables: vec![],
            subentries: vec![],
        }
    }

    pub fn push_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub typ: ValueType,
    pub value: String,
}

impl Variable {
    pub fn new(key: String, typ: ValueType, value: String) -> Self {
        Variable {
            key,
            typ,
            value,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ValueType {
    Str,
    Num,
    Bool,
    Unknown,
}

pub fn check_unknown_value_type(val: &String) -> Result<ValueType, ScanError> {
    let bool_re = Regex::new(r"^TRUE|FALSE$").unwrap();
    let num_re = Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap();
    if bool_re.is_match(val) {
        Ok(ValueType::Bool)
    } else if num_re.is_match(val) {
        Ok(ValueType::Num)
    } else {
        Ok(ValueType::Str)
    }
}

pub fn validate_known_value_type(val: &String, typ: &ValueType) -> Result<(), ScanError> {
    let bool_re = Regex::new(r"^TRUE|FALSE$").unwrap();
    let num_re = Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap();
    match typ {
        ValueType::Num => {
            if !num_re.is_match(val) {
                panic!("not number");
            }
        },
        ValueType::Bool => {
            if !bool_re.is_match(val) {
                panic!("not boolean");
            }
        },
        _ => {},
    }

    Ok(())
}

macro_rules! result_type {
    ($name:ident, $result:ty) => {
        pub type $name = Result<Option<$result>, ScanError>;
    };
}

result_type!(ParseEntryResult, Entry);
result_type!(ParseVariableResult, Variable);
result_type!(ParseHeaderResult, ());

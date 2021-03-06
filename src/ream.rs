use serde::{Serialize, Deserialize};
use crate::scanner::ScanError;

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
    pub value: Value,
}

impl Variable {
    pub fn new(key: String, value: Value) -> Self {
        Variable { key, value }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    String(String),
}


macro_rules! result_type {
    ($name:ident, $result:ty) => {
        pub type $name = Result<Option<$result>, ScanError>;
    };
}

result_type!(ParseEntryResult, Entry);
result_type!(ParseVariableResult, Variable);
result_type!(ParseValueResult, Value);
result_type!(ParseHeaderResult, ());

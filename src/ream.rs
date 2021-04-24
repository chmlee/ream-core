use serde::{Serialize, Deserialize};
use crate::scanner::ScanError;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn get_variable_values(&self) -> Vec<String> {
        let output: Vec<String> = self.variables.to_owned()
            .into_iter()
            .map(|item| item.get_value())
            .collect();

        output
    }

    // TODO: must exist a better way to write this >:(
    pub fn flatten_entry(&self) -> Vec<Vec<String>> {
        let parent = self.get_variable_values();
        if self.subentries.is_empty() { // terminal node
            vec![parent]
        } else {                        // contains subentries
            let subentries = self.subentries.to_owned();
            let mut children: Vec<Vec<String>> = vec![];
            for subentry in subentries {
                let item = subentry.flatten_entry();
                // such ugly code :(
                children.push(item.first().unwrap().to_vec());
            }
            let mut result: Vec<Vec<String>> = vec![];
            for child in children {
                result.push([parent.to_owned(), child].concat());
            }
            result
        }
    }

    pub fn to_csv(&self) -> String {
        let rows = self.flatten_entry();
        let csv_raw = rows.iter().fold(String::new(), |acc, row| acc + &row.join(";") + "\n");
        csv_raw
    }


}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
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

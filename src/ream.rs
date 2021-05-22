use crate::error::*;

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    class: String,
    // parent_class: String,
    level: usize,

    keys: Vec<String>,
    variables: HashMap<String, Value>,
    subentries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value {
    value: ValueBase,
    annotation: String,
    typ: ValueType,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    Str,
    Num,
    Bool,
    Unknown,
    List(Box<ValueType>),
    Ref(Box<ValueType>),
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum ValueBase {
    Str(String),
    Num(String),
    Bool(String),
    Unknown(String),
    List(Box<List>),
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct List {
    item_type: ValueType,
    items: Vec<Value>,
}


impl Entry {
    pub fn new(class: String, level: usize, /*parent_class: String*/) -> Self {
        Entry {
            class,
            // parent_class,
            level,

            keys: Vec::new(),
            variables: HashMap::new(),
            subentries: vec![],
        }
    }

    pub fn push_key(&mut self, key: String) {
        self.keys.push(key);
    }

    pub fn push_subentry(&mut self, subentry: Entry) {
        self.subentries.push(subentry);
    }

    // pub fn parent_class(&self) -> &String {
    //     &self.parent_class
    // }

    pub fn get_variable_values(&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for key in self.keys.clone() { // TODO: clone!
            let item = self.value(&key);
            let item_string = item.to_string();
            output.push(item_string);
        }
        output
    }

    pub fn class(&self) -> &String {
        &self.class
    }

    pub fn keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn insert_variable(&mut self, key: String, value: Value) -> Result<(), ReamError> {
        // also check for duplicate keys
        match self.variables.insert(key, value) {
            None => Ok(()),
            Some(_) => Err(ReamError::DuplicateKeys), // TODO: better error classification
        }
    }

    pub fn value(&self, key: &String) -> Value {
        match self.variables.get(key) {
            Some(value) => value.clone(), // TODO: clone!
            None => unreachable!(), // TODO: un!
        }
    }

    // TODO: must exist a better way to write this >:(
    pub fn flatten_entry(&self) -> Vec<Vec<String>> {
        let parent = self.get_variable_values();
        if self.subentries.is_empty() {
            // terminal node
            vec![parent]
        } else {
            // contains subentries
            let subentries = self.subentries.to_owned();
            let mut children: Vec<Vec<String>> = vec![];
            for subentry in subentries {
                let items = subentry.flatten_entry();
                for item in items {
                    children.push(item);
                }
            }
            let mut result: Vec<Vec<String>> = vec![];
            for child in children {
                result.push([parent.to_owned(), child].concat());
            }
            result
        }
    }

    pub fn to_csv_list(&self) -> Result<Vec<Vec<String>>, ReamError> {
        let rows = self.flatten_entry();
        Ok(rows)
    }

    pub fn to_csv_str(&self) -> Result<String, ReamError> {
        let rows = self.flatten_entry();
        let raw = rows
            .iter()
            .fold(String::new(), |acc, row| acc + &row.join(",") + "\n");
        Ok(raw)
    }

    pub fn to_ast_str_pretty(&mut self) -> Result<String, ReamError> {
        let raw = serde_json::to_string_pretty(&self).unwrap();
        Ok(raw)
    }

    pub fn to_ast_str(&mut self) -> Result<String, ReamError> {
        let raw = serde_json::to_string(&self).unwrap();
        Ok(raw)
    }


}


impl Value {
    pub fn new(value: ValueBase, annotation: String, typ: ValueType) -> Self {
        Self { value, annotation, typ }
    }

    pub fn typ(&self) -> &ValueType {
        &self.typ
    }

    pub fn get_value(&self) -> String {
        self.value.to_string()
    }
}

impl ValueType {
    pub fn size(&self) -> usize {
        match self {
            Self::Unknown => 0,
            Self::Str => 3,
            Self::Num => 3,
            Self::Bool => 4,
            Self::List(u) => (*u).size() + 5,
            Self::Ref(u) => (*u).size() + 4,
        }
    }
}

impl ValueBase {
    pub fn new(val: String, typ: ValueType) -> Result<(Self, ValueType), ReamError> {
        match typ {

            // Value type is not specified.
            // Check for `bool` and `num`.
            // If netiher, return `str`.
            ValueType::Unknown => {
                if is_bool(&val) {
                    Ok((Self::Bool(val), ValueType::Bool))
                } else if is_num(&val) {
                    Ok((Self::Num(val), ValueType::Num))
                } else {
                    Ok((Self::Str(val), ValueType::Str))
                }
            }

            // Value type is specified.
            // Validate value type.
            ValueType::Num => {
                if !is_num(&val) {
                    return Err(ReamError::TypeError(TypeErrorType::InvalidNumber));
                }
                return Ok((Self::Num(val), typ));
            }

            ValueType::Bool => {
                if !is_bool(&val) {
                    return Err(ReamError::TypeError(TypeErrorType::InvalidBoolean));
                }
                return Ok((Self::Bool(val), typ));
            }

            ValueType::Str => return Ok((Self::Str(val), typ)),

            // ValueType::List(t) => return Self::new(val, *t.clone()),

            _ => unreachable!(),
        }
    }

    pub fn new_item(list: List) -> Self {
        ValueBase::List(Box::new(list))
    }

}

impl List {
    pub fn new(typ: ValueType, first_item: Value) -> Self {
        Self {
            item_type: typ,
            items: vec![first_item],
        }
    }

    pub fn push_item(&mut self, new_item: Value) {
        self.items.push(new_item);
    }

    pub fn item_type(&self) -> &ValueType {
        &self.item_type
    }

    pub fn items_as_string(&self) -> String {
        self.items.iter()
            .map(|item| item.get_value())
            .collect::<Vec<String>>()
            .join(";")
    }
}

impl fmt::Display for ValueBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self { // TODO: clone!
            Self::Str(s) => s.clone(),
            Self::Num(s) => s.clone(),
            Self::Bool(s) => s.clone(),
            Self::List(list) => list.items_as_string(),
            _ => "unknown".to_string(),
        };
        write!(f, "{}", string)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.value.clone();
        write!(f, "{}", s)
    }
}


fn is_bool(value: &str) -> bool {
    match value {
        "TRUE" => true,
        "FALSE" => true,
        _ => false,
    }
}

fn is_num(value: &str) -> bool {
    // let re = Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+$").unwrap();
    // re.is_match(value)
    match value.parse::<f64>() {
        Ok(_) => true,
        _ => false,
    }
}

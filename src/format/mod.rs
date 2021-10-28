mod raw;
mod csv;
mod ast;

use crate::error::*;
use crate::decorator::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    class: String,
    level: usize,
    parent_class: Option<String>,

    decorators: Option<Vec<Decorator>>,

    variables: VariableMap,
    subentries: Vec<Entry>,

    keys: Vec<String>,
    ref_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableMap {
    keys: Vec<String>,
    map: HashMap<String, Value>
}

impl VariableMap {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &String) -> Option<&Value> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.map.insert(key, value)
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value {
    value: ValueBase,
    annotation: Option<String>,
    typ: ValueType,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    Str,
    Num,
    Bool,
    Unknown,
    List(Box<ValueType>),
    Ref,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum ValueBase {
    Str(String),
    Num(String),
    Bool(String),
    Unknown(String),
    List(Box<List>),
    Ref(String, String),
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct List {
    item_typ: ValueType,
    items: Vec<Value>,
}

impl Entry {
    pub fn new(class: String, level: usize, parent_class: Option<String>, decorators: Option<Vec<Decorator>>) -> Self {
        Entry {
            class,
            parent_class,
            level,

            decorators,

            variables: VariableMap::new(),
            subentries: vec![],

            keys: Vec::new(),
            ref_keys: Vec::new(),
        }
    }

    pub fn resolve_downstream_ref(&mut self, downstream: &HashMap<String, Vec<VariableMap>>) -> Result<(), ReamError> {
        for key in &self.ref_keys {
            let (ref_class, ref_key) = match self.variables.get(key) {
                Some(v) => {
                    match v.get_base() {
                        ValueBase::Ref(class, key) => (class, key),
                        _ => return Err(ReamError::Placeholder), // TODO: un!
                    }
                }
                _ => return Err(ReamError::Placeholder), // TODO: un!
            };
            let value = match downstream.get(&ref_class) {
                Some(list) => {
                    let mut items: Vec<Value> = Vec::new();
                    for item in list {
                        let new_value = match item.get(&ref_key) {
                            Some(s) => s.clone(),
                            None => return Err(ReamError::ReferenceError(ReferenceErrorType::VariableKeyNotFound)),
                        };
                        items.push(new_value);
                    }
                    // let items = list.iter()
                    //     .map(|hm| hm.get(&ref_key).unwrap().clone()) // TODO: clone!
                    //     .collect::<Vec<Value>>();

                    let (value_base, typ) = List::set_list(items);
                    let value = Value::new(
                        value_base,
                        None,
                        ValueType::List(Box::new(typ)),
                    );

                    value
                },
                None => return Err(ReamError::ReferenceError(
                    ReferenceErrorType::EntryClassNotFound
                )),
            };
            self.variables.insert(key.to_string(), value);
        }
        Ok(())
    }

    pub fn set_ref_key(&mut self, keys: Vec<String>) {
        self.ref_keys = keys;
    }

    pub fn ref_keys(&self) -> Vec<String> {
        self.ref_keys.clone()
    }

    pub fn get_schema(&self) -> EntrySchema {
        EntrySchema::new(self.keys.clone(), self.parent_class.clone())
    }

    pub fn push_key(&mut self, key: String) {
        self.keys.push(key);
    }

    pub fn push_subentry(&mut self, subentry: Entry) {
        self.subentries.push(subentry);
    }

    pub fn get_parent_class(&self) -> Option<String> {
        self.parent_class.clone() // TODO: clone!
    }

    pub fn get_variable_values(&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for key in self.keys.clone() {
            match self.value(&key) {
                Some(item) => {
                    let item_string = item.get_raw();
                    output.push(item_string);
                },
                None => {}
            }
        }
        output
    }

    pub fn class(&self) -> String {
        self.class.clone() // TODO: clone!
    }

    pub fn keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn variable_map(&self) -> VariableMap {
        self.variables.clone() // TODO: clone!
    }

    pub fn insert_variable(&mut self, key: String, value: Value) -> Result<(), ReamError> {
        // also check for duplicate keys
        match self.variables.insert(key, value) {
            None => Ok(()),
            Some(_) => Err(ReamError::DuplicateKeys), // TODO: better error classification
        }
    }

    pub fn value(&self, key: &String) -> Option<&Value> {
        self.variables.get(key)
    }


}

impl Value {
    pub fn new(value: ValueBase, annotation: Option<String>, typ: ValueType) -> Self {
        Self {
            value,
            annotation,
            typ,
        }
    }

    pub fn typ(&self) -> &ValueType {
        &self.typ
    }

    pub fn get_base(&self) -> ValueBase {
        self.value.clone()
    }

    pub fn get_annotation(&self) -> Option<String> {
        self.annotation.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.get_raw()
    }

    pub fn get_base_and_typ(&self) -> (ValueBase, ValueType) {
        (self.value.clone(), self.typ.clone()) // TODO: clone!
    }

    pub fn get_raw(&self) -> String {
        self.value.get_raw()
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
            Self::Ref => 3,
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

            ValueType::List(t) => return Err(ReamError::Placeholder),

            _ => return Err(ReamError::Placeholder),
        }
    }

    pub fn new_item(list: List) -> Self {
        ValueBase::List(Box::new(list))
    }

    pub fn new_ref(class: String, key: String) -> Self {
        Self::Ref(class, key)
    }

    pub fn get_raw(&self) -> String {
        match self {
            Self::Str(s) => s.to_string(),
            Self::Num(s) => s.to_string(),
            Self::Bool(s) => s.to_string(),
            Self::Unknown(s) => s.to_string(),
            Self::List(list) => list.items_as_string(),
            Self::Ref(_, _) => "UnreachablePlaceholder".to_string(),
        }
    }

}

impl List {
    pub fn new(typ: ValueType, first_item: Value) -> Self {
        Self {
            item_typ: typ,
            items: vec![first_item],
        }
    }

    pub fn push_item(&mut self, new_item: Value) {
        self.items.push(new_item);
    }

    pub fn item_type(&self) -> &ValueType {
        &self.item_typ
    }

    pub fn items_as_string(&self) -> String {
        self.items
            .iter()
            .map(|item| item.get_value())
            .collect::<Vec<String>>()
            .join(";")
    }

    pub fn set_list(items: Vec<Value>) -> (ValueBase, ValueType) {
        let (_, typ) = items[0].get_base_and_typ();
        let item_typ = ValueType::List(Box::new(typ.clone())); // TODO: clone!
        let list = Self { item_typ, items };
        (ValueBase::List(Box::new(list)), typ)
    }
}

// impl fmt::Display for ValueBase {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let string = match self {
//             // TODO: clone!
//             Self::Str(s) => s.clone(),
//             Self::Num(s) => s.clone(),
//             Self::Bool(s) => s.clone(),
//             Self::List(list) => list.items_as_string(),
//             _ => "unknown".to_string(),
//         };
//         write!(f, "{}", string)
//     }
// }


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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntrySchema {
    keys: Vec<String>,
    parent_class: Option<String>,
}

impl EntrySchema {
    pub fn new(keys: Vec<String>, parent_class: Option<String>) -> Self {
        Self { keys, parent_class }
    }

    pub fn keys(&self) -> Vec<String> {
        self.keys.clone() // TODO: clone!
    }

    pub fn get_parent_class(&self) -> Option<String> {
        self.parent_class.clone() // TODO: clone
    }
}

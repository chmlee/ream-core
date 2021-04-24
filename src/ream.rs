use serde::{Serialize, Deserialize};
use crate::scanner::ScanError;
use regex::Regex;
use std::iter::FromIterator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub class: String,
    pub level: usize,

    pub variables: Vec<Variable>,
    pub subentries: Vec<Entry>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VariableVec {
//     content: Vec<Variable>,
// }

// impl VariableVec {
//     pub fn new() -> VariableVec {
//         VariableVec {
//             content: vec![],
//         }
//     }

//     pub fn push(&mut self, var: Variable) {
//         self.content.push(var);
//     }

//     pub fn len(&self) -> usize {
//         self.content.len()
//     }

//     pub fn get_index(&self, i: usize) -> Variable {
//         self.content[i].to_owned()
//     }
// }

// impl IntoIterator for VariableVec {
//     type Item = Variable;
//     type IntoIter = VariableVecIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         VariableVecIntoIterator {
//             items: self,
//             index: 0,
//         }
//     }
// }

// pub struct VariableVecIntoIterator {
//     items: VariableVec,
//     index: usize,
// }

// impl Iterator for VariableVecIntoIterator {
//     type Item = Variable;
//     fn next(&mut self) -> Option<Variable> {
//         let result = if self.index < self.items.len() {
//             Some(self.items.get_index(self.index))
//         } else {
//             None
//         };
//         self.index += 1;
//         result
//     }
// }

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

    // pub fn get_variable_values(&self) -> Vec<String> {
    //     let output: Vec<String> = self.variables.to_owned()
    //         .into_iter()
    //         .map(|item| item.get_value())
    //         .collect();

    //     output
    // }

    // // TODO: must exist a better way to write this >:(
    // pub fn get_subentries(&self) -> Vec<Vec<String>> {
    //     let subentry = self.subentries.to_owned();
    //     let mut result: Vec<Vec<String>> = vec![];
    //     for i in subentry.content {
    //         result.push(i.get_variable_values());

    //     }
    //     result
    // }

    // pub fn flatten(&self) -> Vec<Vec<String>> {
    //     let mut result: Vec<Vec<String>> = vec![];
    //     let parent = self.get_variable_values();
    //     let children = self.get_subentries();
    //     for child in children {
    //         result.push([parent.to_owned(), child].concat());
    //     }
    //     result
    // }

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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct EntryVec {
//     content: Vec<Entry>,
// }

// impl EntryVec {

//     pub fn new() -> Self {
//         EntryVec {
//             content: vec![],
//         }
//     }

//     pub fn push(&mut self, entry: Entry) {
//         self.content.push(entry);
//     }

//     pub fn len(&self) -> usize {
//         self.content.len()
//     }

//     pub fn get_index(&self, i: usize) -> Entry {
//         self.content[i].to_owned()
//     }
// }

// impl IntoIterator for EntryVec {
//     type Item = Entry;
//     type IntoIter = EntryVecIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         EntryVecIntoIterator {
//             items: self,
//             index: 0,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct EntryVecIntoIterator {
//     items: EntryVec,
//     index: usize,
// }

// impl Iterator for EntryVecIntoIterator {
//     type Item = Entry;
//     fn next(&mut self) -> Option<Entry> {
//         let result = if self.index < self.items.len() {
//             Some(self.items.get_index(self.index))
//         } else {
//             None
//         };
//         self.index += 1;
//         result
//     }
// }

// impl FromIterator<Entry> for EntryVecIntoIterator {
//     fn from_iter<I: IntoIterator<Item=EntryVec>(iter: I) -> Self {
//         let mut c = Vec<String>::new()
//     }
// }


macro_rules! result_type {
    ($name:ident, $result:ty) => {
        pub type $name = Result<Option<$result>, ScanError>;
    };
}

result_type!(ParseEntryResult, Entry);
result_type!(ParseVariableResult, Variable);
result_type!(ParseHeaderResult, ());

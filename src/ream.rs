use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    class: String,
    level: usize,

    variables: Vec<Variable>,
    subentries: Vec<Entry>,
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

    pub fn push_subentry(&mut self, subentry: Entry) {
        self.subentries.push(subentry);
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

    pub fn to_csv_list(&self) -> Result<Vec<Vec<String>>, ReamError> {
        let rows = self.flatten_entry();
        Ok(rows)
    }

    pub fn to_csv_str(&self) -> Result<String, ReamError> {
        let rows = self.flatten_entry();
        let raw = rows.iter().fold(String::new(), |acc, row| acc + &row.join(",") + "\n");
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variable {
    key: String,
    typ: ValueType,
    value: String,
    annotation: String,
}

impl Variable {
    pub fn new(key: String, typ: ValueType, value: String, annotation: String) -> Self {
        Variable {
            key,
            typ,
            value,
            annotation,
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
    List(Box<ValueType>),
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
        _ => false
    }
}


pub fn check_unknown_value_type(val: &String) -> Result<ValueType, ReamError> {
    if is_bool(val) {
        Ok(ValueType::Bool)
    } else if is_num(val) {
        Ok(ValueType::Num)
    } else {
        Ok(ValueType::Str)
    }
}

pub fn validate_known_value_type(val: &String, typ: &ValueType) -> Result<(), ReamError> {
    match typ {
        ValueType::Num => {
            if !is_num(val) {
                return Err(ReamError::TypeError(TypeErrorType::InvalidNumber))
            }
        },
        ValueType::Bool => {
            if !is_bool(val) {
                return Err(ReamError::TypeError(TypeErrorType::InvalidBoolean))
            }
        },
        _ => {},
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReamError {
    ScanError(ScanErrorType),
    ParseError(ParseErrorType),
    TypeError(TypeErrorType),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeErrorType {
    UnknownType,
    InvalidNumber,
    InvalidBoolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParseErrorType {
    MissingHeaderLevel,
    MissingIdentifier,
    MissingVariable,
    MissingSubentry,
    MissingValue,
    MissingToken,
    MissingColon,
    WrongHeaderLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScanErrorType {
    InvalidToken,
    // ToFewSpaces,
    MissingValue,
    MissingKey,
    MissingClass,
    MissingEOL,
    MissingColon,
    InvalidType,
    WrongHeaderLevel,
}

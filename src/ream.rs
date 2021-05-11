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
    value: ReamValue,
    annotation: String,
}

impl Variable {
    pub fn new(key: String, value: ReamValue, annotation: String) -> Self {
        Variable {
            key,
            value,
            annotation,
        }
    }

    pub fn get_value(&self) -> String {
        self.value.get_value()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum ReamValue {
    Str(String),
    Num(String),
    Bool(String),
    Unknown(String),
    List(Vec<ReamValue>),
}

impl ReamValue {
    pub fn get_value(&self) -> String {
        match self {
            Self::Str(s) => s.to_string(),
            Self::Num(s) => s.to_string(),
            Self::Bool(s) => s.to_string(),
            _ => unreachable!(),
            // Unknown(String),
            // List(Vec<ReamValue>),
        }
    }

    pub fn new(val: String, typ: ValueType) -> Result<Self, ReamError> {
        match typ {
            // Value type is not specified.
            // Check for `bool` and `num`.
            // If netiher, return `str`.
            ValueType::Unknown => {
                if is_bool(&val) {
                    Ok(ReamValue::Bool(val))
                } else if is_num(&val) {
                    Ok(ReamValue::Num(val))
                } else {
                    Ok(ReamValue::Str(val))
                }
            },
            // Value type is specified.
            // Validate value type.
            ValueType::Unit(UnitType::Num) => {
                if !is_num(&val) {
                    return Err(ReamError::TypeError(TypeErrorType::InvalidNumber))
                }
                return Ok(ReamValue::Num(val))
            },
            ValueType::Unit(UnitType::Bool) => {
                if !is_bool(&val) {
                    return Err(ReamError::TypeError(TypeErrorType::InvalidBoolean))
                }
                return Ok(ReamValue::Bool(val))
            },
            ValueType::Unit(UnitType::Str) => {
                return Ok(ReamValue::Str(val))
            },
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    Unknown,
    Unit(UnitType),
    List(UnitType),
}

impl ValueType {
    pub fn size(&self) -> usize {
        match self {
            Self::Unknown => 0,
            Self::Unit(u) => u.size(),
            Self::List(u) => u.size() + 5,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum UnitType {
    Str,
    Num,
    Bool,
}

impl UnitType {
    pub fn size(&self) -> usize {
        match *self {
            Self::Str => 3,
            Self::Num => 3,
            Self::Bool => 4,
        }
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
        _ => false
    }
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

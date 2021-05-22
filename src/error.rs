use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReamError {
    ScanError(ScanErrorType),
    ParseError(ParseErrorType),
    TypeError(TypeErrorType),
    ReferenceError(ReferenceErrorType),
    SchemaError(SchemaErrorType),
    DuplicateKeys, // TODO: better error classification
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchemaErrorType {
    IncorrectParentClass,
    IncorrectKeys,
    IncorrectClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReferenceErrorType {
    ReferenceNotFound,
    InvalidReference,
    EntryClassNotFound,
    VariableKeyNotFound,
    IncompatibleTypes,
    DuplicateKeys,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeErrorType {
    UnknownType,
    InvalidNumber,
    InvalidBoolean,
    HeterogeneousList,
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

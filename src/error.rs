use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReamError {
    ScanError(ScanErrorType),
    ParseError(ParseErrorType),
    TypeError(TypeErrorType),
    ReferenceError(ReferenceErrorType),
    SchemaError(SchemaErrorType),
    DecoratorError(DecoratorErrorType),
    DuplicateKeys, // TODO: better error classification
    Placeholder,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchemaErrorType {
    IncorrectParentClass,
    IncorrectKeys,
    IncorrectClass,
    IncorrectSchema, // TODO: need to be more specific
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
    MissingDecorator,
    InvalidType,
    WrongHeaderLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DecoratorErrorType {
    InvalidDecorator,
}

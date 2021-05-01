#[derive(Debug)]
pub enum ReamError {
    ScanError(ScanErrorType),
    ParseError(ParseErrorType),
    TypeError(TypeErrorType),
}

#[derive(Debug)]
pub enum TypeErrorType {
    UnknownType,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ScanErrorType {
    InvalidToken,
    ToFewSpaces,
    MissingValue,
    MissingKey,
    MissingClass,
    MissingEOL,
    MissingColon,
    WrongHeaderLevel,
}

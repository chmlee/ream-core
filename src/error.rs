pub enum ReamError {
    ScanError(ScanErrorType),
    ParseError(ParserErrorType),
}

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

/// Members are different strategy specific errors
#[derive(Debug, Eq, PartialEq)]
pub enum StrategyError {
    MessageTooLarge(String),
    CannotConvertMessageLength,
    CannotGroupMessageLength,
    CannotConvertMessageBytes,
    GeneralMediaError(CoverMediaError)
}

/// Members are different cover media specific errors
#[derive(Debug, Eq, PartialEq)]
pub enum CoverMediaError {
    BadFile(String),
    GenerateFileFailure,
    WriteFileFailure
}

/// Members are different generic errors
#[derive(Debug, Eq, PartialEq)]
pub enum GenericError {

}

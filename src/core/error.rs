/// Members are different strategy specific errors
#[derive(Debug, Eq, PartialEq)]
pub enum StegoStrategyError {
    MessageTooLarge(String),
    CannotConvertMessageLength,
    CannotGroupMessageLength,
    CannotConvertMessageBytes,
    GeneralMediaError(StegoCoverMediaError)
}

/// Members are different cover media specific errors
#[derive(Debug, Eq, PartialEq)]
pub enum StegoCoverMediaError {
    BadFile(String),
    GenerateFileFailure,
    WriteFileFailure
}

/// Members are different generic errors
#[derive(Debug, Eq, PartialEq)]
pub enum StegoGenericError {

}

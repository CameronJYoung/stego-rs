#[derive(Debug)]
pub enum EncodingError {
    BadFile(String),
    MessageTooLarge(String),
    GenerateFileFailure,
    WriteFileFailure
}

#[derive(Debug)]
pub enum DecodingError {
    BadFile(String),
    CannotGroupMessageLength,
    CannotConvertMessageLength,
    CannotConvertMessageBytes
}
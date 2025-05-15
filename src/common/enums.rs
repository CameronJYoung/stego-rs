#[derive(Debug)]
pub enum EncodingError {
    BadFile(String),
    MessageTooLarge(String),
    GenerateFileFailure,
    WriteFileFailure
}
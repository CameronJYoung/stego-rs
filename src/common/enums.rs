#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedFileType {
    Png,
    WavPcm,
}

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
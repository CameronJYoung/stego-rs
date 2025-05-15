pub enum EncodingError {
    BadFile(String),
    MessageTooLarge(String)
}
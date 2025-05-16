use crate::common::enums::{EncodingError, SupportedFileType};
use crate::png::encode::encode_png;

pub struct StegoEncoder {
    file_type: SupportedFileType
}

impl StegoEncoder {
    pub fn new(file_type: SupportedFileType) -> Self {
        Self { file_type }
    }

    pub fn encode(&self, message: &str, image_path: &str, output_path: &str) -> Result<(), EncodingError> {
        match self.file_type {
            SupportedFileType::PNG => encode_png(message, image_path, output_path)
        }
    }
}

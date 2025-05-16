use crate::common::enums::{DecodingError, SupportedFileType};
use crate::png::decode::decode_png;

pub struct StegoDecoder {
    file_type: SupportedFileType
}

impl StegoDecoder {
    pub fn new(file_type: SupportedFileType) -> Self {
        Self { file_type }
    }

    pub fn decode(&self, image_path: &str) -> Result<String, DecodingError> {
        match self.file_type {
            SupportedFileType::PNG => decode_png(image_path)
        }
    }
}

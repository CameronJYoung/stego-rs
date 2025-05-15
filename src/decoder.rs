use crate::common::enums::{DecodingError};
use crate::image_type::SupportedImageType;
use crate::png::decode::decode_png;

pub struct ImageDecoder {
    image_type: SupportedImageType
}

impl ImageDecoder {
    pub fn new(image_type: SupportedImageType) -> Self {
        Self { image_type }
    }

    pub fn decode(&self, image_path: &str) -> Result<String, DecodingError> {
        match self.image_type {
            SupportedImageType::PNG => decode_png(image_path)
        }
    }
}

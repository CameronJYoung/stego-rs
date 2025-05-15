use crate::common::enums::EncodingError;
use crate::image_type::SupportedImageType;
use crate::png::encode::encode_png;

pub struct ImageEncoder {
    image_type: SupportedImageType
}

impl ImageEncoder {
    pub fn new(image_type: SupportedImageType) -> Self {
        Self { image_type }
    }

    pub fn encode(&self, message: &str, image_path: &str, output_path: &str) -> Result<(), EncodingError> {
        match self.image_type {
            SupportedImageType::PNG => encode_png(message, image_path, output_path)
        }
    }
}

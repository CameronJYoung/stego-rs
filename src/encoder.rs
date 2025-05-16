use crate::common::enums::{EncodingError, SupportedFileType};
use crate::png::encode::encode_png;
use crate::wav_pcm::encode::encode_wav_pcm;

pub struct StegoEncoder {
    file_type: SupportedFileType
}

impl StegoEncoder {
    pub fn new(file_type: SupportedFileType) -> Self {
        Self { file_type }
    }

    pub fn encode(&self, message: &str, input_path: &str, output_path: &str) -> Result<(), EncodingError> {
        match self.file_type {
            SupportedFileType::Png => encode_png(message, input_path, output_path),
            SupportedFileType::WavPcm => encode_wav_pcm(message, input_path, output_path),
        }
    }
}

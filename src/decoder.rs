use crate::common::enums::{DecodingError, SupportedFileType};
use crate::png::decode::decode_png;
use crate::wav_pcm::decode::decode_wav_pcm;

pub struct StegoDecoder {
    file_type: SupportedFileType
}

impl StegoDecoder {
    pub fn new(file_type: SupportedFileType) -> Self {
        Self { file_type }
    }

    pub fn decode(&self, input_path: &str) -> Result<String, DecodingError> {
        match self.file_type {
            SupportedFileType::Png => decode_png(input_path),
            SupportedFileType::WavPcm => decode_wav_pcm(input_path)
        }
    }
}

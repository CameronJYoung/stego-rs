use hound::WavReader;

use crate::common::enums::DecodingError;
use crate::common::utils::byte_from_lsb_group;

pub fn decode_wav_pcm(input_path: &str) -> Result<String, DecodingError> {
    // Get the wav reader using the hound crate
    let mut wav_reader = match WavReader::open(input_path) {
        Ok(w) => w,
        Err(e) => return Err(DecodingError::BadFile(e.to_string()))
    };

    // Get PCM data in bytes
    let mut pcm_bytes: Vec<u8> = Vec::new();
    for sample in wav_reader.samples::<i32>() { // TODO: This will fail if the supplied audio is not 32 bits. When we implement file type specific config we should make this configurable
        let sample = match sample {
            Ok(s) => s,
            Err(e) => return Err(DecodingError::BadFile(e.to_string()))
        };

        pcm_bytes.extend(&sample.to_le_bytes());
    };

    // Get the message length from the first 32 bytes of image data
    let message_length_bytes = pcm_bytes[..32].to_vec();

    // Get message length in bytes
    let message_length: u32 = match byte_from_lsb_group(message_length_bytes, 32) {
        Some(bytes) => {
            let byte_slice: Result<[u8; 4], _> = bytes.as_slice().try_into();

            match byte_slice {
                Ok(n) => u32::from_be_bytes(n),
                Err(_) => {
                    return Err(DecodingError::CannotConvertMessageLength);
                }
            }
        },
        None => {
            return Err(DecodingError::CannotGroupMessageLength);
        }
    };

    // Get remaining message bytes
    let message_bytes: Vec<u8> = pcm_bytes[32..(32 + (message_length * 8) as usize)].to_vec();

    // convert message bytes into a string and return it
    match byte_from_lsb_group(message_bytes, (message_length * 8) as usize) {
        Some(bytes) => {
            match String::from_utf8(bytes) {
                Ok(s) => Ok(s),
                Err(_) => {
                    Err(DecodingError::CannotConvertMessageBytes)
                }
            }
        },
        None => {
            Err(DecodingError::CannotConvertMessageBytes)
        },
    }
}

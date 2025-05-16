use crate::common::enums::{DecodingError};
use crate::common::utils::byte_from_lsb_group;

pub fn decode_png(image_path: &str) -> Result<String, DecodingError> {
    // Get the PNG using the image library
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => {
            return Err(DecodingError::BadFile(e.to_string()))
        }
    };

    // Extract RGB data
    let rgb_img = image.to_rgba8();

    // Convert to a nicer type
    let rgb_data_bytes: Vec<u8> = rgb_img.pixels().flat_map(|p| p.0).collect();

    // Get the message length from the first 32 bytes of image data
    let message_length_bytes = rgb_data_bytes[..32].to_vec();

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
    let message_bytes: Vec<u8> = rgb_data_bytes[32..(32 + (message_length * 8) as usize)].to_vec();

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

use image::{RgbaImage};
use crate::common::enums::EncodingError;
use crate::common::utils::{bytes_to_bits, update_byte_lsb};

pub fn encode_png(message: &str, image_path: &str, output_path: &str) -> Result<(), EncodingError> {
    // Get the PNG using the image library
    let image = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => {
            return Err(EncodingError::BadFile(e.to_string()))
        }
    };

    // Extract RGB data
    let rgb_img = image.to_rgba8();

    // Convert to a nicer type
    let rgb_data_bytes: Vec<u8> = rgb_img.pixels().flat_map(|p| p.0).collect();

    // Get actual message bytes
    let message_bytes = message.as_bytes();

    // Work out the bit length of the message
    //
    // TODO: Document this properly - casting to u32 will cause issues with very large messages, this being restricted is fine but should probably be done upstream.
    let message_byte_length = message_bytes.len() as u32;

    // Convert message size to bytes
    let message_size_bytes: [u8; 4] = message_byte_length.to_be_bytes();

    // Calculate byte size for message length + message
    let encoded_data_bytes_count: u32 =  4 + message_byte_length;

    // Calculate size requirements
    //
    // We are using the single least significant bit per byte of image data (colour channel).
    // This means we work out the amount of bytes required by doing "encoded_data_bytes_count * 8"
    let image_size_requirements_bytes: u32 = encoded_data_bytes_count * 8;

    // Check there's enough bytes
    if rgb_data_bytes.len() < image_size_requirements_bytes as usize {
        return Err(EncodingError::MessageTooLarge(
            format!("Image has {} bytes. Message requires {y} bytes.", rgb_data_bytes.len(), y = image_size_requirements_bytes)
        ));
    };

    // Get all encoded data in bytes
    let mut encoded_data_bytes: Vec<u8> = Vec::with_capacity(encoded_data_bytes_count as usize);
    encoded_data_bytes.extend_from_slice(&message_size_bytes);
    encoded_data_bytes.extend_from_slice(&message_bytes);

    // Convert encoded byte data to bits
    let encoded_data_bits = bytes_to_bits(&encoded_data_bytes);

    // Create vector to store final data
    let mut final_encoded_data: Vec<u8> = Vec::new();

    for i in 0..rgb_data_bytes.len() {
        if i < image_size_requirements_bytes as usize {
            final_encoded_data.push(update_byte_lsb(rgb_data_bytes[i], encoded_data_bits[i]));
        } else {
            final_encoded_data.push(rgb_data_bytes[i]);
        }
    }

    let new_image: RgbaImage = match RgbaImage::from_raw(image.width(), image.height(), final_encoded_data) {
        Some(i) => i,
        None => return Err(EncodingError::GenerateFileFailure)
    };

    match new_image.save(output_path) {
        Ok(_) => Ok(()),
        Err(_) => return Err(EncodingError::WriteFileFailure)
    }
}

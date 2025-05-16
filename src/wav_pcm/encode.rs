use hound::{WavReader, WavSpec, WavWriter};
use crate::common::enums::EncodingError;
use crate::common::utils::{bytes_to_bits, update_byte_lsb};

pub fn encode_wav_pcm(message: &str, input_path: &str, output_path: &str) -> Result<(), EncodingError> {
    // Get the wav reader using the hound crate
    let mut wav_reader = match WavReader::open(input_path) {
        Ok(w) => w,
        Err(e) => return Err(EncodingError::BadFile(e.to_string()))
    };

    // Get PCM data in bytes
    let mut pcm_bytes: Vec<u8> = Vec::new();
    for sample in wav_reader.samples::<i32>() { // TODO: This will fail if the supplied audio is not 32 bits. When we implement file type specific config we should make this configurable
        let sample = match sample {
            Ok(s) => s,
            Err(e) => return Err(EncodingError::BadFile(e.to_string()))
        };

        pcm_bytes.extend(&sample.to_le_bytes());
    }


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
    let wav_size_requirements_bytes: u32 = encoded_data_bytes_count * 8;

    // Check there's enough bytes
    if pcm_bytes.len() < wav_size_requirements_bytes as usize {
        return Err(EncodingError::MessageTooLarge(
            format!("Image has {} bytes. Message requires {y} bytes.", pcm_bytes.len(), y = wav_size_requirements_bytes)
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

    for i in 0..pcm_bytes.len() {
        if i < wav_size_requirements_bytes as usize {
            final_encoded_data.push(update_byte_lsb(pcm_bytes[i], encoded_data_bits[i]));
        } else {
            final_encoded_data.push(pcm_bytes[i]);
        }
    }

    // Create new wav spec from original wav
    let new_wav_spec = WavSpec {
        ..wav_reader.spec()
    };

    // Create a hound writer
    let mut new_wav_writer = match WavWriter::create(output_path, new_wav_spec) {
        Ok(w) => w,
        Err(_) => return Err(EncodingError::WriteFileFailure)
    };

    // Chunk the final encoded data into samples and add it to the writer
    for chunk in final_encoded_data.chunks_exact(4) {
        let bytes: [u8; 4] = match chunk.try_into() {
            Ok(t) => t,
            Err(_) => return Err(EncodingError::GenerateFileFailure)
        };

        let sample = i32::from_le_bytes(bytes);

        if let Err(_) = new_wav_writer.write_sample(sample) {
            return Err(EncodingError::GenerateFileFailure);
        }
    }

    // Write the file
    if let Err(_) = new_wav_writer.finalize() {
        return Err(EncodingError::GenerateFileFailure);
    }

    Ok(())
}

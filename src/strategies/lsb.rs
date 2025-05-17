use crate::core::bit_utils::{byte_from_lsb_group, bytes_to_bits, update_byte_lsb};
use crate::core::cover_media::CoverMedia;
use crate::core::error::{StegoStrategyError};
use crate::core::strategy::StegoStrategy;

pub struct LsbStrategy;

impl LsbStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl StegoStrategy for LsbStrategy {
    fn encode(&self, message: &str, media: &mut dyn CoverMedia) -> Result<(), StegoStrategyError> {
        // Read bytes from cover media
        let cover_media_bytes = media.read_bytes();

        // Get bytes from message
        let message_bytes = message.as_bytes();

        // Work out the bit length of the message
        //
        // TODO: Read Below
        // Casting to u32 will cause issues with very large messages, this being restricted is fine
        // but should probably be done upstream.
        let message_byte_length = message_bytes.len() as u32;

        // Convert message length to bytes
        let message_length_bytes = message_byte_length.to_be_bytes();

        // Calculate byte size for message length + message
        let encoded_data_bytes_count: u32 =  4 + message_byte_length;

        // Calculate size requirements
        //
        // We are using the single least significant bit per byte of media data.
        // This means we work out the amount of bytes required by doing "encoded_data_bytes_count * 8"
        let minimum_media_size_requirement_bytes: u32 = encoded_data_bytes_count * 8;

        // Check there's enough bytes
        if cover_media_bytes.len() < minimum_media_size_requirement_bytes as usize {
            return Err(
                StegoStrategyError::MessageTooLarge(
                    format!(
                        "Media has {} bytes. Message requires {y} bytes.",
                        cover_media_bytes.len(),
                        y = minimum_media_size_requirement_bytes
                    )
                )
            )
        };

        // Get all encoded data in bytes
        let mut encoded_data_bytes: Vec<u8> = Vec::with_capacity(encoded_data_bytes_count as usize);
        encoded_data_bytes.extend_from_slice(&message_length_bytes);
        encoded_data_bytes.extend_from_slice(&message_bytes);

        // Convert encoded byte data to bits
        let encoded_data_bits = bytes_to_bits(&encoded_data_bytes);

        // Create vector to store final data
        let mut final_encoded_data: Vec<u8> = Vec::new();

        for i in 0..cover_media_bytes.len() {
            if i < minimum_media_size_requirement_bytes as usize {
                final_encoded_data.push(update_byte_lsb(cover_media_bytes[i], encoded_data_bits[i]));
            } else {
                final_encoded_data.push(cover_media_bytes[i]);
            }
        }

        // Write new bytes to passed in media
        match media.write_bytes(final_encoded_data.as_slice()) {
            Ok(_) => Ok(()),
            Err(e) => Err(StegoStrategyError::GeneralMediaError(e))
        }
    }

    fn decode(&self, media: &dyn CoverMedia) -> Result<String, StegoStrategyError> {
        // Read bytes from cover media
        let cover_media_bytes = media.read_bytes();

        // Get the message length from the first 32 bytes of image data
        let message_length_bytes = cover_media_bytes[..32].to_vec();

        // Get message length in bytes
        let message_length: u32 = match byte_from_lsb_group(message_length_bytes, 32) {
            Some(bytes) => {
                let byte_slice: Result<[u8; 4], _> = bytes.as_slice().try_into();

                match byte_slice {
                    Ok(n) => u32::from_be_bytes(n),
                    Err(_) => {
                        return Err(StegoStrategyError::CannotConvertMessageLength);
                    }
                }
            },
            None => {
                return Err(StegoStrategyError::CannotGroupMessageLength);
            }
        };

        // Get remaining message bytes
        let message_bytes: Vec<u8> = cover_media_bytes[32..(32 + (message_length * 8) as usize)].to_vec();

        // convert message bytes into a string and return it
        match byte_from_lsb_group(message_bytes, (message_length * 8) as usize) {
            Some(bytes) => {
                match String::from_utf8(bytes) {
                    Ok(s) => Ok(s),
                    Err(_) => {
                        Err(StegoStrategyError::CannotConvertMessageBytes)
                    }
                }
            },
            None => {
                Err(StegoStrategyError::CannotConvertMessageBytes)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::error::StegoCoverMediaError;
    use super::*;

    struct MockMedia {
        data: Vec<u8>
    }

    impl MockMedia {
        pub fn new() -> Self {
            Self {
                data: vec![0xAB; 1024], // Add Kib of predictable binary
            }
        }
    }

    impl CoverMedia for MockMedia {
        fn read_bytes(&self) -> &[u8] {
            &self.data
        }

        fn write_bytes(&mut self, new_bytes: &[u8]) -> Result<(), StegoCoverMediaError> {
            self.data.clear();
            self.data.extend_from_slice(new_bytes);
            Ok(())
        }

        fn clone_with_bytes(&self, new_bytes: &[u8]) -> Result<Box<dyn CoverMedia>, StegoCoverMediaError> {
            Ok(Box::new(MockMedia {
                data: new_bytes.to_vec(),
            }))
        }
    }

    #[test]
    fn test_encode_success() {
        let mut media = MockMedia::new();
        let lsb_strategy = LsbStrategy::new();

        // TODO: Think of a better test to actually assert a value without writing out a stupid amount of bytes
        assert_eq!(lsb_strategy.encode("test", &mut media).is_ok(), true);
    }

    #[test]
    fn test_encode_message_to_big_failure() {
        let mut media = MockMedia::new();
        let lsb_strategy = LsbStrategy::new();

        let large_str: &str = &"💩".repeat(38); // 8 * 38 bytes should be over the 1kb test amount

        assert_eq!(
            lsb_strategy.encode(large_str, &mut media).err(),
            Some(
                StegoStrategyError::MessageTooLarge("Media has 1024 bytes. Message requires 1248 bytes.".to_string())
            )
        );
    }

    #[test]
    fn test_decode_success() {
        let mut media = MockMedia::new();
        let lsb_strategy = LsbStrategy::new();

        let message = "testing-testing-123";

        lsb_strategy.encode(message, &mut media).unwrap();

        assert_eq!(
            lsb_strategy.decode(&mut media).unwrap(),
            message
        );
    }
}
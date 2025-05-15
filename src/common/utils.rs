pub fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    bytes.iter()
        .flat_map(|byte| (0..8).rev().map(move |i: u8| (byte >> i) & 1))
        .collect()
}

pub fn update_byte_lsb(original_byte: u8, message_bit: u8) -> u8 {
    (original_byte & 0b1111_1110) | (message_bit & 1)
}

pub fn get_byte_lsb(byte: u8) -> u8 {
    byte & 1
}

// TODO: Think of a better name for this
//
// This function will accept a vector of bytes, group them into 8, get the lsb for each byte and put
// each new byte in a vector. It will do this for a maximum number of times. For PNG, the encoded message
// length is the first 32 bytes of image data. Using that as an example, i'd want to call byte_from_lsb_group(v, 32)
// and should return you 4 bytes which contains the length of the encoded message. Using that you can
// call this function again to get the message.
pub fn byte_from_lsb_group(bytes: Vec<u8>, byte_limit: usize) -> Option<Vec<u8>> {
    // The byte limit should always be a multiple of 8 (as it will take 8 bytes of image data for 1 byte of String data)
    if byte_limit % 8 != 0 {
        return None;
    };

    // This is the amount of expected bytes for the String data
    let expected_byte_amount = byte_limit / 8;

    // Get all the bits by mapping over the bytes and getting the LSB for each one
    let bits: Vec<u8> = bytes.iter().map(|v| get_byte_lsb(*v)).collect::<Vec<u8>>();

    let mut output = Vec::new();

    // Group our bits into eights
    for chunk in bits.chunks(8).take(byte_limit / 8) {
        let mut byte = 0u8;

        for (i, bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }

        output.push(byte);
    };

    // If the output length is not what is expected return nothing
    if output.len() != expected_byte_amount {
        return None;
    };

    Some(output)
}
#[cfg(test)]
mod tests {
    use super::*;

    // bytes_to_bits
    #[test]
    fn test_bytes_to_bits_happy_path() {
        let byte_slice: &[u8] = &[0b10101111, 0b11000011];

        let bit_vector: Vec<u8> = vec![
            1, 0, 1, 0, 1, 1, 1, 1,
            1, 1, 0, 0, 0, 0, 1, 1
        ];

        assert_eq!(bytes_to_bits(&byte_slice), bit_vector);
    }

    // update_byte_lsb
    #[test]
    fn test_update_byte_lsb_happy_path_1() {
        let byte: u8 = 0b10101111;
        let bit: u8 = 0b0;

        let expected: u8 = 0b10101110;

        assert_eq!(update_byte_lsb(byte, bit), expected);
    }

    #[test]
    fn test_update_byte_lsb_happy_path_same_0() {
        let byte: u8 = 0b10101110;
        let bit: u8 = 0b1;

        let expected: u8 = 0b10101111;

        assert_eq!(update_byte_lsb(byte, bit), expected);
    }

    #[test]
    fn test_update_byte_lsb_happy_path_same_value() {
        let byte: u8 = 0b10101111;
        let bit: u8 = 0b1;

        let expected: u8 = 0b10101111;

        assert_eq!(update_byte_lsb(byte, bit), expected);
    }

    // get_byte_lsb
    #[test]
    fn test_get_byte_lsb_happy_path_1() {
        let byte: u8 = 0b10101111;
        let expected_bit: u8 = 0b1;

        assert_eq!(get_byte_lsb(byte), expected_bit);
    }

    #[test]
    fn test_get_byte_lsb_happy_path_2() {
        let byte: u8 = 0b10101110;
        let expected_bit: u8 = 0b0;

        assert_eq!(get_byte_lsb(byte), expected_bit);
    }

    // byte_from_lsb_group
    #[test]
    fn test_byte_from_lsb_group_happy_path() {
        let bytes = vec![
            0b10101110,
            0b10101111,
            0b10101111,
            0b10101111,
            0b10101111,
            0b10101110,
            0b10101111,
            0b10101110
        ];

        let expected_bytes = vec![
            0b01111010
        ];

        assert_eq!(byte_from_lsb_group(bytes, 8), Some(expected_bytes))
    }
}
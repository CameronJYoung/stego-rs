pub fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    bytes.iter()
        .flat_map(|byte| (0..8).rev().map(move |i: u8| (byte >> i) & 1))
        .collect()
}

pub fn update_byte_lsb(original_byte: u8, message_bit: u8) -> u8 {
    (original_byte & 0b1111_1110) | (message_bit & 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // bytes_to_bits
    #[test]
    fn test_bytes_to_bits_happy_path() {
        let byte_slice: &[u8] = &[0b10101111, 0b11000011];

        let bit_vector: Vec<u8> = vec![
            1, 0, 1, 0, 1, 1, 1, 1, // 0b10101111
            1, 1, 0, 0, 0, 0, 1, 1  // 0b11000011
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
}
pub fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    bytes.iter()
        .flat_map(|byte| (0..8).rev().map(move |i: u8| (byte >> i) & 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // bytes_to_bits
    #[test]
    fn test_happy_path() {
        let byte_slice: &[u8] = &[0b10101111, 0b11000011];

        let bit_vector: Vec<u8> = vec![
            1, 0, 1, 0, 1, 1, 1, 1, // 0b10101111
            1, 1, 0, 0, 0, 0, 1, 1  // 0b11000011
        ];

        assert_eq!(bytes_to_bits(&byte_slice), bit_vector)
    }
}
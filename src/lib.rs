pub fn bit_to_bytes(bits: Vec<u8>) -> [u8; 5] {
    let mut bytes = [0u8; 5];
    bits.iter()
        .enumerate()
        .for_each(|(i, x)| {
            let byte_index = i / 8;
            let bit_position = i % 8;
            if byte_index < bytes.len() {
                bytes[byte_index] |= x << bit_position;
            }
        });
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bit_convert() {
        let bits = [0,0,0,0,1,1,1,1,
                    0,0,0,0,1,1,1,1,
                    0,0,0,0,1,1,1,1,
                    0,0,0,0,1,1,1,1,
                    0,0,0,0,1,1,1,1];
        assert_eq!(bit_to_bytes(bits.to_vec()), [240, 240, 240, 240, 240]);
    }
}
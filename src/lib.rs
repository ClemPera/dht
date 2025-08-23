pub fn bit_to_bytes(bits: Vec<u8>) -> [u8; 5] {
  //! Converts bits received to bytes
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

pub fn checksum(bytes: [u8; 5]) -> Result<(), ()> {
  //! Is checksum passed
  let mut total  = 0;

  bytes.iter()
    .for_each(|x| {total = total + x});

  //Remove the checksum value from checksum total
  total = total - bytes[4];

  if total == bytes[4]{
    return Ok(())
  }else{
    return Err(())
  }
}

pub fn convert_to_decimal(bytes: [u8; 5]) -> [f32; 2]{
  //! convert bytes to float
  [
    bytes[0] as f32 + bytes[1] as f32 / 10.0,
    bytes[2] as f32 + bytes[3] as f32 / 10.0
  ]
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
  
  #[test]
  fn test_checksum() {
    let bytes = [1,2,3,4,10];
    
    assert_eq!(checksum(bytes), Ok(()));
  }

  #[test]
  fn test_convert_to_decimal() {
    let bytes = [1,2,3,4,10];
    assert_eq!(convert_to_decimal(bytes), [1.2, 3.4]);
  }
}
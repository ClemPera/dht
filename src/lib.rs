//Converts bits received to bytes
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

//Checksum is passed
pub fn checksum(bytes: [u8; 5]) -> Result<(), ()> {
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
}
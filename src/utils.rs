pub struct DhtData {
  pub temperature: f32,
  pub humidity: f32
}

pub fn bits_to_bytes(bits: Vec<u8>) -> [u8; 5] {
  //! Converts bits to bytes (MSB)
  let mut bytes = [0u8; 5];

  bits.iter()
  .enumerate()
  .for_each(|(i, x)| {
    let byte_index = i / 8;
    let bit_position = 7 - (i % 8); //Flip to MSB

    if byte_index < bytes.len() {
      bytes[byte_index] |= x << bit_position;
    }
  });

  bytes
}

pub fn checksum(bytes: [u8; 5]) -> Result<(), ()> {
  //! Is checksum passed
  let mut total: u16 = 0;

  bytes.iter().for_each(|x| {total = total + *x as u16;});

  //Remove the checksum value from checksum total
  total = total - (bytes[4] as u16);

  if total == (bytes[4] as u16){
    Ok(())
  }else{
    Err(())
  }
}

pub fn convert_to_data_struct(bytes: [u8; 5]) -> DhtData{
  //! convert bytes to float

  DhtData {
    humidity: bytes[0] as f32 + bytes[1] as f32 / 10.0,
    temperature: bytes[2] as f32 + bytes[3] as f32 / 10.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_bit_convert() {
    let bits = [0,0,0,0,1,1,1,1,
                1,0,0,0,1,1,1,1,
                1,1,1,1,0,0,0,0,
                0,0,0,0,0,0,0,0,
                1,1,1,1,1,1,1,1];

    assert_eq!(bits_to_bytes(bits.to_vec()), [15, 143, 240, 0, 255]);
  }
  
  #[test]
  fn test_checksum() {
    let bytes = [1,2,3,4,10];
    
    assert_eq!(checksum(bytes), Ok(()));
  }

  #[test]
  fn test_convert_to_data_struct() {
    let bytes = [54,0,24,1,79];
    let data = convert_to_data_struct(bytes);

    assert_eq!(data.humidity, 54.0);
    assert_eq!(data.temperature, 24.1);
  }
}
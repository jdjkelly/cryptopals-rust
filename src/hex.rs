fn value(c: u8) -> Result<u8, String> {
  match c {
      b'A'..=b'F' => Ok(c - b'A' + 10),
      b'a'..=b'f' => Ok(c - b'a' + 10),
      b'0'..=b'9' => Ok(c - b'0'),
      _ => Err("out of bounds".to_string())
  }
}

pub fn decode(hex: Vec<u8>) -> Result<Vec<u8>, String> {
  hex.chunks(2).enumerate().map(|(_, pair)| {
      Ok(value(pair[0])? << 4 | value(pair[1])?)
  }).collect()
}

pub fn encode(bytes: Vec<u8>) -> Result<String, String> {
  let alphabet: Vec<char> = "0123456789abcdef".chars().collect();

  let mut result: String = String::new();

  for chunk in bytes {
    // we need to split each 8 bit sequence into two 4 bit sequences
    // for left, just shove everything left by 4
    // for right, BITAND the last 4 values
    let left = chunk >> 4;
    let right = chunk & 0xF;

    result.push(alphabet[left as usize]);
    result.push(alphabet[right as usize]);
  };

  Ok(result)
}
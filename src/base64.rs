pub fn encode(bytes: Vec<u8>) -> String {
  let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect();

  let mut result: String = String::new();

  for (_, chunk) in bytes.chunks(3).enumerate() {
      match chunk.len() {
          3 => {
              result.push(alphabet[(chunk[0] >> 2) as usize]);
              result.push(alphabet[(((chunk[0] & 0x03) << 4) | ((chunk[1] & 0xF0) >> 4)) as usize]);
              result.push(alphabet[(((chunk[1] & 0x0F) << 2) | ((chunk[2] & 0xC0) >> 6)) as usize]);
              result.push(alphabet[(chunk[2] & 0x3F) as usize]);
          }
          _ => println!("todo")
      }
  }

  result
}
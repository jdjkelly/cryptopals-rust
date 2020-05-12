use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    hex_to_convert: String
}


fn value(c: u8) -> Result<u8, String> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err("out of bounds".to_string())
    }
}

fn convert(hex: Vec<u8>) -> Result<Vec<u8>, String> {
    hex.chunks(2).enumerate().map(|(_, pair)| {
        Ok(value(pair[0])? << 4 | value(pair[1])?)
    }).collect()
}

fn to_base64(bytes: Vec<u8>) -> String {
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

fn main() {
    let args = Cli::from_args();

    let hex = args.hex_to_convert;

    println!("hex input:     {}", hex);

    if hex.len() % 2 != 0 {
        panic!("Wrong length");
    }

    let to_vec: Vec<u8> = hex.into_bytes();

    let as_bytes = convert(to_vec).unwrap();
    
    println!("base64 output: {}", to_base64(as_bytes));
}

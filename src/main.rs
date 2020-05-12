
mod hex;
mod base64;

fn main() {
    println!("cryptopals-rust: why not run the test?")
}

// Take a hex string and transform it into base64 encoding, with byte manipulation alone
fn s1c1(hex: String) -> String {
    if hex.len() % 2 != 0 {
        panic!("Wrong length");
    }

    let to_vec: Vec<u8> = hex.into_bytes();

    let as_bytes = hex::decode(to_vec).unwrap();

    base64::encode(as_bytes)
}

#[test]
fn test_s1c1() {
    assert_eq!(
        s1c1(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    )
}

// XOR a hex string with another one
fn s1c2(a: String, b: String) -> String {
    let (a_bytes, b_bytes) = (a.into_bytes(), b.into_bytes());
    let (decoded_a, decoded_b) = (hex::decode(a_bytes).unwrap(), hex::decode(b_bytes).unwrap());

    let xor_op = decoded_a.iter().enumerate().map(|(i, byte)| {
        byte ^ decoded_b[i]
    }).collect();

    hex::encode(xor_op).unwrap()
}

#[test]
fn test_s1c2() {
    assert_eq!(
        s1c2(String::from("1c0111001f010100061a024b53535009181c"), String::from("686974207468652062756c6c277320657965")),
        "746865206b696420646f6e277420706c6179"
    )
}
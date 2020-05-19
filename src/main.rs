use std::fs::File;
use std::io::{self, BufRead};

mod hex;
mod base64;
mod freq;

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

// Breaking Single-byte XOR cipher
fn s1c3(hex: String) -> String {
    let decoded_hex = hex::decode(hex.into_bytes()).unwrap();

    let mut best: (Vec<char>, f32) = (vec![' '], 0f32);

    for candidate in 0x0..=0xFF {
        // create possible candidates by xoring each possible full byte
        let xor_op: Vec<u8> = decoded_hex.iter().map(|byte| {
            byte ^ candidate
        }).collect();

        // convert the candidate to ascii
        let to_char: Vec<char> = xor_op.iter().map(|&n| {
            n as char
        }).collect();

        // convert the vector of characters into a string for convenience
        // let result: String = to_char.into_iter().collect();

        let score = freq::score(&to_char);
        
        if best.1 < score {
            best = (to_char, score);
        }
    };

    best.0.into_iter().collect()
}

#[test]
fn test_s1c3() {
    assert_eq!(s1c3(String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")), String::from("Cooking MC\'s like a pound of bacon"));
}


fn s1c4() {
    let data = File::open("./data/4.txt").unwrap();
    let lines = io::BufReader::new(data).lines();

    let mut best_of_each: Vec<String> = Vec::new();

    for line in lines {
        best_of_each.push(s1c3(line.unwrap().to_string()));
    };

    let mut best: (Vec<char>, f32) = (vec![' '], 0f32);

    for n in best_of_each {
        // n.as_bytes()
        let score = freq::score(n);
        
        if best.1 < score {
            best = (n, score);
        }
    };

    best
}

#[test]
fn test_s1c4() {
    s1c4();
}
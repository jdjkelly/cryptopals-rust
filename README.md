# 🦀 cryptopals-rust

Rust solutions to the https://cryptopals.com/ problem sets, using nothing but `std`, no external libraries**, and an emphasis on from-scratch solutions.

** With the exception of `structopt` to add CLI friendliness, but this is not essential.

# Set 1
* ✔️ Convert hex to base64
  - Implemented with a custom `hex` module
  - Solution relies upon bitwise operations (&, |, <<, >>) - for both decoding stringified hex values and shifting the resulting `vec[u8]` into 6-bit numbers, to be re-encoded by base64
* Fixed XOR
* Single-byte XOR cipher
* Detect single-character XOR
* Implement repeating-key XOR
* Break repeating-key XOR
* AES in ECB mode
* Detect AES in ECB mode

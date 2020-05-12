# 🦀 cryptopals-rust

Rust solutions to the https://cryptopals.com/ problem sets, using nothing but `std`, no external libraries**, and an emphasis on from-scratch solutions.

** With the exception of `structopt` to add CLI friendliness, but this is not essential.

# Set 1
* ✔️ Convert hex to base64
  - Implemented with a custom `hex` module
  - Solution relies upon bitwise operations (&, |, <<, >>) - for both decoding stringified hex values and shifting the resulting `vec[u8]` into 6-bit numbers, to be re-encoded by base64
* ✔️ Fixed XOR
  - Convert each string to a `vec[u8]` using existing `hex` module from previous exercise
  - Iterate through each byte in A, use XOR operator `^` on the same index position in B
  - Re-encode result back to hex by splitting each u8 into two 4-bit values with bitwise operations
* ⚠️ Single-byte XOR cipher
  - Not a complete answer, but the last part is trivial (char frequence counting)
  - Just iterate through the given hex with the possible values 0x0 through 0xFF
  - Take the resulting Vec<Vec<u8>> and convert to ascii
  - (todo) Analyze the results for frequency of letters matching expected values in english
* Detect single-character XOR
* Implement repeating-key XOR
* Break repeating-key XOR
* AES in ECB mode
* Detect AES in ECB mode

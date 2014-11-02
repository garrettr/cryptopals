extern crate core;
use core::iter::range_step;

// To compile and test this code:
//
//   $ rustc --test 1-1-base64.rs
//   $ ./1-1-base64
//
// Will automatically run the unit test below, which was given in the problem statement

#[test]
fn test_conversion() {
    let hex = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64 = convert_hex_to_base64(hex);
    assert!(base64.as_slice() == b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

fn hex_table(c: u8) -> u8 {
    match c {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'a' => 10,
        b'b' => 11,
        b'c' => 12,
        b'd' => 13,
        b'e' => 14,
        b'f' => 15,
        _ => fail!("Encountered invalid character in hexadecimal string"),
    }
}

fn base64_table(c: u8) -> u8 {
    match c {
        0 => b'A',
        1 => b'B',
        2 => b'C',
        3 => b'D',
        4 => b'E',
        5 => b'F',
        6 => b'G',
        7 => b'H',
        8 => b'I',
        9 => b'J',
        10 => b'K',
        11 => b'L',
        12 => b'M',
        13 => b'N',
        14 => b'O',
        15 => b'P',
        16 => b'Q',
        17 => b'R',
        18 => b'S',
        19 => b'T',
        20 => b'U',
        21 => b'V',
        22 => b'W',
        23 => b'X',
        24 => b'Y',
        25 => b'Z',
        26 => b'a',
        27 => b'b',
        28 => b'c',
        29 => b'd',
        30 => b'e',
        31 => b'f',
        32 => b'g',
        33 => b'h',
        34 => b'i',
        35 => b'j',
        36 => b'k',
        37 => b'l',
        38 => b'm',
        39 => b'n',
        40 => b'o',
        41 => b'p',
        42 => b'q',
        43 => b'r',
        44 => b's',
        45 => b't',
        46 => b'u',
        47 => b'v',
        48 => b'w',
        49 => b'x',
        50 => b'y',
        51 => b'z',
        52 => b'0',
        53 => b'1',
        54 => b'2',
        55 => b'3',
        56 => b'4',
        57 => b'5',
        58 => b'6',
        59 => b'7',
        60 => b'8',
        61 => b'9',
        62 => b'+',
        63 => b'/',
        _ => fail!("Encountered invalid value for conversion to base64"),
    }
}

fn convert_hex_to_base64(hex: &[u8]) -> Vec<u8> {

    // 1. Decode hex input into a vector of bytes
    let mut bytes = Vec::new();
    // Each pair of hex digits encodes one byte
    for i in range_step(0, hex.len(), 2) {
        let mut byte : u8 = 0;
        for j in range(0, 2) {
            let c = hex[i+j];
            let nibble = hex_table(c);
            byte = byte | (nibble << 4*(1-j));
        }
        bytes.push(byte);
    }

    // 2. Encode base64 output
    let mut base64 = Vec::new();
    // Encode bytes in 3-byte groups
    for i in range_step(0, bytes.len(), 3) {
        let mut chunk : u32 = 0;
        for j in range(0, 3) {
            let mut byte = 0;
            // When the number of bytes to encode is not divisible by 3,
            // automatically pad out with 0's
            if i+j < bytes.len() {
                byte = bytes[i+j];
            }
            chunk = chunk | (byte as u32 << 8*(2-j));
        }

        // Encode each 3-byte group into 4 Base64 characters,
        // six bits at a time (2^6 = 64)
        for j in range(0, 4) {
            let six_bit_val : u8 = ((chunk >> 6*(3-j)) & 0b00111111u as u32) as u8;
            base64.push(base64_table(six_bit_val));
        }
    }

    base64
}


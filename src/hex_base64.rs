///! The Algorithm
/// Convert hex string into raw bytes - each hexadecimal symbol can be represented by four bits. 
/// If two hexadecimal digits are taken together, they can be "packed" into an 8-bit byte by putting 
/// the first character's four bits in one half of the byte, and the second character's four bits 
/// in the other half of the byte - by Bit-shifting and the bitwise-OR operator to pack the two nibbles into a single byte
/// First, every bit in the first nibble is shifted 4 spots to the left, Second, the "high nibble" bits and the "low nibble" 
/// bits are joined together using the bitwise OR operator
/// 
/// Convert bytes to Base64 - base64 has 64 characters, 6 bits each. 3 bytes of data (24 bits) are mapped to 4 base64 chars.
/// for example
/// input bytes:    S           u           n
/// hex:            0x53        0x75        0x6E
/// binary:         01010011    01110101    01101110
/// 6 bit parts:    010100      110111      010101      101110
/// decimal value:  20          55          21          46
/// base64 char:    U           3           V           u
/// 
/// if the input is not divisible by 3, the bytes buffer is filled with zeros until it is divisible by 3
/// 
/// Bytes to Hex string - 1 byte is 2 hex digits. (b >> 4) & 0x0F - this operation shifts the high 4 bits down 
/// (>> is right shift) and logical ANDs it with 0000 1111 so that the result is an integer equal 
/// to the high 4 bits of the byte (first hex digit). The rest is basically just using the zero or a character 
/// as a starting point and shifting up to the correct character. The first if statement covers all the digits 0-9, 
/// and the second covers all digits 10-15 (a-f in hex)


const B64_TABLE: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

 
pub fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {

    Some(hex
    .chars()
    .collect::<Vec<char>>()
    .chunks_exact(2)
    .map(|chunk| {
        let first_byte = chunk[0].to_digit(16).unwrap();
        let second_byte = chunk[1].to_digit(16).unwrap();
        (first_byte << 4 | second_byte) as u8
    })
    .collect::<Vec<u8>>())
    
}

pub fn base64_encode(bytes: &[u8]) -> String {
    let mut i = 0;
    let index_of_last_complete_triple = ((bytes.len() / 3) * 3) as usize;
    let mut chars: Vec<u8> = vec![];
    /* handle triples of input characters per loop */
    while i < index_of_last_complete_triple {
        // take the six most significant bits of first byte
        let a = *bytes.get(i+0).unwrap_or(&0) as usize;
        let b = *bytes.get(i+1).unwrap_or(&0) as usize;
        let c = *bytes.get(i+2).unwrap_or(&0) as usize;

        // take the six most significant bits of first byte and find the corresponding base64 char
        chars.push(B64_TABLE[a >> 2]);
        // take the lower two bits of first byte and place them to the most significant bit locations
        // take the four most significant bits of second byte and place them to the four lowest bit locations
        chars.push(B64_TABLE[(a & 0x03) << 4 | (b & 0xF0) >> 4]);
        // take the four less significant bits of second byte and place them to the most significant bit locations
        // take the two most significant bits of third byte and place them to the two lowest bit locations
        chars.push(B64_TABLE[(b & 0x0F) << 2 | (c & & 0xC0) >> 6]);
        // take the six less significant bits of third byte
        chars.push(B64_TABLE[c & 0x3F]);

        i += 3;
    }

    if i < bytes.len(){
        // last triple incomplete, either one or two input characters 'missing'
        // get first index value, always available
        let x = *bytes.get(i).unwrap_or(&0) as usize;
        // get second index value, if second input byte of last triple not available 'fill up with zeros'
        let y = *bytes.get(i+1).unwrap_or(&0) as usize;
        // encode first byte of last incomplete triple
        chars.push(B64_TABLE[x >> 2]);
        chars.push(B64_TABLE[(x & 0x03) << 4 | (y & 0xF0) >> 4]);

        if i+1 < bytes.len() {
            // only one byte 'missing', encode last character = second byte in last triple
            chars.push(B64_TABLE[(y & 0x0F) << 2]);
        } else {
            // two bytes 'missing', add one padding character
            chars.push('=' as u8);
        }
        chars.push('=' as u8);
    }

    String::from_utf8(chars).unwrap()

}

pub fn to_hex_str(bytes: &[u8]) -> String {
        
    bytes.iter()
      .flat_map(|&b| [b >> 4, b & 0b1111])
      .map(|b| match b {
        0..=9 => (b + b'0') as char,
        10..=16 => (b + b'a' - 10) as char,
        _ => unreachable!(),
      })
      .collect()
}
  

#[cfg(test)]
mod set1_one_tests{
    use super::*;
    use std::str;
    #[test]
    fn hex_to_base64(){
        const INPUT: &str ="7072656d6174757265206f7074696d697a6174696f6e2069732074686520726f6f74206f6620616c6c206576696c21";
        const OUTPUT: &str = "cHJlbWF0dXJlIG9wdGltaXphdGlvbiBpcyB0aGUgcm9vdCBvZiBhbGwgZXZpbCE=";
        let bytes = hex_to_bytes(INPUT).unwrap();
        if let Ok(s) = str::from_utf8(&bytes) {
            assert_eq!(s, "premature optimization is the root of all evil!");
            println!("Intermediate bytes representation: {}", s);
        }
        let base64_str = base64_encode(&bytes);
        println!("Found:    {}", base64_str);
        println!("Expected: {}", OUTPUT);
        assert_eq!(base64_str, OUTPUT);
    }
}
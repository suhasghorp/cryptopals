///! The Algorithm
/// this is very easy - we are XORing 2 byte arrays, byte by byte.
/// and then converting resulting array back to base64

use crate::challenge1;

pub fn xor_bytes(a : &str, b:&str) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    let a = challenge1::hex_to_bytes(a).unwrap();
    let b = challenge1::hex_to_bytes(b).unwrap();
    a.iter().zip(b).map(|(b1,b2)| b1^b2).collect::<Vec<_>>()
}

pub fn xor(a : &str, b:&str) -> String {
    let c = xor_bytes(a, b);
    challenge1::to_hex_str(&c)
}

#[cfg(test)]
mod set1_two_tests{
    use super::*;    
    #[test]
    fn xor_test(){
        assert_eq!(
            xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"),            
            String::from("746865206b696420646f6e277420706c6179"),
        );
        
    }
}
///! The Algorithm
/// this is very easy - we are XORing 2 byte arrays, byte by byte.
/// and then converting resulting array back to base64

use crate::hex_base64;

pub fn xor(a : &str, b:&str) -> String {
    let a = hex_base64::hex_to_bytes(a).unwrap();
    let b = hex_base64::hex_to_bytes(b).unwrap();
    let c = a.iter().zip(b).map(|(b1,b2)| b1^b2).collect::<Vec<_>>();
    hex_base64::to_hex_str(&c)
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
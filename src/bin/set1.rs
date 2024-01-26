use cryptopals::hex_base64::{hex_to_bytes, base64_encode};
use cryptopals::xor::xor;
use cryptopals::xor_cipher::xor_cipher;
use std::str;

const INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";



fn main() {
    println!("HEX TO BASE64");
    let bytes = hex_to_bytes(INPUT).unwrap();
    if let Ok(s) = str::from_utf8(&bytes) {
        println!("Intermediate bytes representation: {}", s);
    }
    let base64_str = base64_encode(&bytes);    
    println!("Found:    {}", base64_str);
    println!("Expected: {}", OUTPUT);    
    println!();

    println!("XOR");
    println!("Found:    {}", xor("1c0111001f010100061a024b53535009181c","686974207468652062756c6c277320657965"));            
    println!("Expected: {}", String::from("746865206b696420646f6e277420706c6179"));
    println!();

    println!("XOR_CIPHER");
    let ans_tuple = xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let decoded = String::from_utf8(ans_tuple.unwrap().0.to_vec()).unwrap();
    println!("Found:    {}", decoded);            
    println!("Expected: {}", String::from("Cooking MC's like a pound of bacon"));
    println!();
    
  }

use openssl::symm::{decrypt, Cipher};

pub fn aes_ecb_dec(key:&[u8], ciphertext : &[u8]) -> Vec<u8> {
    decrypt(Cipher::aes_128_ecb(), key, None, &ciphertext).unwrap()
}
    

#[cfg(test)]
mod set1_seven_tests{
    use super::*;  
    use std::path::Path;
    use crate::utility;
    #[test]
    fn aes_ecb_dec_test(){
        let ciphertext = utility::from_base64_file(Path::new("/home/sghorp/rustprojects/cryptopals/data/challenge7.txt"));
        let key:&[u8] = b"YELLOW SUBMARINE";
        let plaintext = aes_ecb_dec(key,&ciphertext);
        println!("{}", String::from_utf8(plaintext).unwrap());
        assert_eq!(1,1);
    }
}
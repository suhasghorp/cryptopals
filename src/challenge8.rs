const BLOCK_SIZE:usize = 16;

pub fn bytes_to_chunks(bytes:&[u8], chunk_size:usize) -> Vec<Vec<&[u8]>>{
    (0..bytes.len())
    .step_by(chunk_size)
    .map(|x|&bytes[x..x + chunk_size])
    .collect()
}

#[cfg(test)]
mod set1_eight_tests{
    use super::*;  
    use std::path::Path;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::challenge1;
    #[test]
    fn aes_ecb_dec_test(){
        let path = "/home/sghorp/rustprojects/cryptopals/data/challenge8.txt";
        let mut ciphertexts = Vec::new();
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {        
            ciphertexts.push(challenge1::hex_to_bytes(&line.unwrap().trim()).unwrap());
        }

        for (idx, ciphertext) in ciphertexts.iter().enumerate(){
            let num_block = ciphertext.len() / BLOCK_SIZE;
        }
    }
}
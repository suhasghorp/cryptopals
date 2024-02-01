use std::collections::HashSet;

pub fn bytes_to_chunks(bytes:&[u8], chunk_size:usize) -> HashSet<&[u8]> {
    
    (0..bytes.len())
    .step_by(chunk_size)
    .map(|ind| &bytes[ind..ind + chunk_size])
    .collect::<Vec<_>>()
    .into_iter()
    .collect::<HashSet<&[u8]>>()
    
}

#[cfg(test)]
mod set1_eight_tests{
    use super::*;  
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::challenge1;
    const BLOCK_SIZE:usize = 16;

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
            let num_blocks = ciphertext.len() / BLOCK_SIZE;
            let unique_chunks = bytes_to_chunks(&ciphertext, BLOCK_SIZE);
            let num_unique_blocks = unique_chunks.len();
            let repeated_blocks = num_blocks - num_unique_blocks;
            if repeated_blocks == 0{
                continue;
            }
            println!("Line {} has {} repeated blocks and is likely using ECB", idx, repeated_blocks);
        }
        assert_eq!(1,1);
    }
}
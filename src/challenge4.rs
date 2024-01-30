//Set 1 Challenge 4

use crate::challenge3;
use crate::challenge1;
use crate::challenge3::ScoredGuess;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines() -> Vec<Vec<u8>> {
    let path = "/home/sghorp/rustprojects/cryptopals/data/challenge4.txt";
    let mut content = Vec::new();
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {        
        content.push(challenge1::hex_to_bytes(&line.unwrap().trim()).unwrap());
    }
    content
}

pub fn xor_single_char() -> ScoredGuess {
    let mut overall_best = ScoredGuess::new();
    for line in get_lines() {
        let candidate = challenge3::crack_xor_cipher(&line);
        overall_best = min(overall_best, candidate);
    }

    if overall_best.cipher_text == None {
        panic!("no ciphertext found (this should never happen!");
    }

    overall_best  
}

#[cfg(test)]
mod set1_four_tests{
    use super::*;    
    #[test]
    fn xor_single_char_test(){
        let overall_best = xor_single_char();
        println!("lines.index(overall_best.ciphertext)={:?}", overall_best.cipher_text.unwrap());
        println!("overall_best.key={:?}", overall_best.key);
        assert_eq!("Now that the party is jumping\n".to_owned(), overall_best.plain_text.unwrap());
    }
}
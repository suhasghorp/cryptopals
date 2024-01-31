// Set 1 Challenge 6

use itertools::Itertools;
use crate::challenge3::crack_xor_cipher;
use crate::challenge3::ScoredGuess;

const MAX_KEYSIZE:usize = 40;

// this function will calc weights at COMPILE TIME
const fn gen_precomputed_weights() -> [(u8, u8);256] {
    let mut weights : [(u8,u8);256] = [(0u8,0u8);256];
    let mut i : usize = 0;
    while i < 256 {
        weights[i].0 = i as u8;
        weights[i].1 = (i as u8).count_ones() as u8;
        i += 1;
    }
    weights
}

pub static PRECOMUPTED_HAMMING_WTS: [(u8, u8); 256] = gen_precomputed_weights();

pub fn hamming_distance(a : &[u8], b: &[u8]) -> u32 {
    let test = a.iter().zip(b.iter())
    .map(|(a,b)| a ^ b)
    .collect::<Vec<u8>>();

    let mut sum: u32 = 0;
    for x in test {
        sum += PRECOMUPTED_HAMMING_WTS.iter().filter(|(a,_)| x == *a).map(|x| x.1 as u32).next().unwrap() ;
    }
    
    sum
}

pub fn get_score(bytes : &[u8],size:usize) -> f32 {
    let chunks = vec![&bytes[..size],&bytes[size..2*size],&bytes[2*size..3*size],&bytes[3*size..4*size]];
    chunks.into_iter().combinations(2)
    .map(|v| hamming_distance(v.get(0).unwrap(), v.get(1).unwrap()))
    .sum::<u32>() as f32/(6.0 * size as f32)
}

pub fn guess_keysize(bytes : &[u8], num_guesses : usize) -> Vec<(f32,usize)>{
    let mut v :Vec<(f32,usize)> = (2..=MAX_KEYSIZE)
    .map(|size| (get_score(bytes, size),size))
    .collect();
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    v[0..num_guesses].to_vec()    
}

pub fn crack_repeating_key_xor(bytes : &[u8],keysize:usize) -> (f32, Vec<u8>) {
    let chunks = (0..keysize)
    .map(|n|
        bytes[n..].iter()
        .enumerate()
        .filter(|&(i, _)| i % keysize == 0)
        .map(|(_, e)| *e)
        .collect::<Vec<_>>()
    )
    .collect::<Vec<Vec<_>>>();
    

    let cracks = chunks.iter()
    .map(|chunk| crack_xor_cipher(chunk))
    .collect::<Vec<ScoredGuess>>();

    let combined_score:f32 = cracks.iter().map(|guess| guess.score).sum::<f32>()/keysize as f32;
    let key :Vec<u8> = cracks.iter().map(|guess| guess.key.unwrap()).collect();

    (combined_score,key)
    
}

#[cfg(test)]
mod set1_six_tests{
    use super::*;  
    use crate::challenge5::repeating_key_xor;
    use std::path::Path;
    use crate::utility;
    #[test]
    fn xor_break_test(){
        let x = hamming_distance(b"this is a test", b"wokka wokka!!!");
        println!("{}", x);

        let ciphertext = utility::from_base64_file(Path::new("/home/sghorp/rustprojects/cryptopals/data/challenge6.txt"));
        let keysizes = guess_keysize(&ciphertext, 5);
        println!("Best key size guesses (confidence, size):");
        println!("{:?}", keysizes);
        let mut candidates :Vec<(f32,Vec<u8>)> = Vec::new();
        for (_, size) in keysizes.iter(){
            let temp = crack_repeating_key_xor(&ciphertext, *size);
            candidates.push(temp);
        }
        candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let best_candidate = candidates.get(0).unwrap();
        let best_key = &best_candidate.1;

        println!("Top guess:");
        println!("{:?}",String::from_utf8_lossy(&best_key));
        println!("plaintext =\n");

        let x = repeating_key_xor(&ciphertext, &best_key, );
        println!("decoded paragraph : {}", String::from_utf8(x).unwrap());
        assert_eq!("Terminator X: Bring the noise", String::from_utf8_lossy(best_key));
    }
}
    


use crate::xor_cipher;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines() -> Vec<String> {
    let path = "/home/sghorp/rustprojects/cryptopals/data/challenge4.txt";
    let mut content = Vec::new();
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        content.push(line.unwrap());
    }
    content
}

pub fn xor_single_char() -> String {
    get_lines().iter()
    .map(|line|xor_cipher::xor_cipher(line).unwrap())
    .min_by_key(|(_, score)| *score)
    .map(|(decoded, _)| String::from_utf8(decoded).unwrap())
    .unwrap()    
}

#[cfg(test)]
mod set1_four_tests{
    use super::*;    
    #[test]
    fn xor_single_char_test(){
        let ans = xor_single_char();
        assert_eq!("Now that the party is jumping\n".to_owned(), ans);
    }
}
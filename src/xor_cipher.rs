
use crate::hex_base64;
use std::collections::HashMap;

// Source:
// Lee, E. Stewart. "Essays about Computer Security" (PDF). University of Cambridge Computer Laboratory. p. 181.
pub static EXPECTED_FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), // Whitespace
    (b'.', 6.57),  // Others
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
];


fn is_control(u: u8) -> bool {
    u < 0x20 || u == 0x7F  //space or del char
}
fn is_alphabetic(u: u8) -> bool {
    //A..Z || a..z
    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
}
fn get_character_counts(v: &[u8]) -> HashMap<u8, f32> {
    let mut counts: HashMap<u8, f32> = HashMap::new();
    for &c in v.iter() {
        if is_control(c) {
            continue;
        }
        let key = if is_alphabetic(c) {
            c.to_ascii_lowercase()
        } else if c == b' ' || c == b'\t' {
            b' '
        } else {
            b'.'
        };

        let count = counts.entry(key).or_insert(0f32);
        *count += 1f32;
    }
    counts
}

pub fn get_score(vec :&[u8]) -> u32 {
    let mut score : u32 = 0;
    let length = vec.len() as f32;
    let counts = get_character_counts(vec);
    if !vec.is_ascii() {
        return std::u32::MAX;
    }

    if vec.iter().any(|&c| is_control(c) && c != b'\n') {            
        return std::u32::MAX; 
    }

    for (c, expected_score) in EXPECTED_FREQUENCIES.iter(){
        let expected_count = expected_score / 100f32 * length;
        let &actual_count = counts.get(&c).unwrap_or(&0f32);
        score += (expected_count - actual_count).powi(2) as u32;
    }

    score   
}

pub fn xor_cipher(input : &str) -> Option<(Vec<u8>, u32)> {
    let bytes = hex_base64::hex_to_bytes(input).unwrap();
    //there are 34 bytes here
    
    let vec:Vec<_> = (0u8..=255)
    .map(|key| bytes.iter().map(|&b| b^key).collect::<Vec<u8>>()).collect(); 

    // the above results in a vec of 255 length of vec of u8 of 34 length like below (only 10 out 255 shown)
    /*
        [237, 193, 193, 197, 199, 192, 201, 142, 227, 237, 137, 221, 142, 194, 199, 197, 203, 142, 207, 142, 222, 193, 219, 192, 202, 142, 193, 200, 142, 204, 207, 205, 193, 192], 
        [236, 192, 192, 196, 198, 193, 200, 143, 226, 236, 136, 220, 143, 195, 198, 196, 202, 143, 206, 143, 223, 192, 218, 193, 203, 143, 192, 201, 143, 205, 206, 204, 192, 193], 
        [227, 207, 207, 203, 201, 206, 199, 128, 237, 227, 135, 211, 128, 204, 201, 203, 197, 128, 193, 128, 208, 207, 213, 206, 196, 128, 207, 198, 128, 194, 193, 195, 207, 206], 
        [226, 206, 206, 202, 200, 207, 198, 129, 236, 226, 134, 210, 129, 205, 200, 202, 196, 129, 192, 129, 209, 206, 212, 207, 197, 129, 206, 199, 129, 195, 192, 194, 206, 207], 
        [225, 205, 205, 201, 203, 204, 197, 130, 239, 225, 133, 209, 130, 206, 203, 201, 199, 130, 195, 130, 210, 205, 215, 204, 198, 130, 205, 196, 130, 192, 195, 193, 205, 204], 
        [224, 204, 204, 200, 202, 205, 196, 131, 238, 224, 132, 208, 131, 207, 202, 200, 198, 131, 194, 131, 211, 204, 214, 205, 199, 131, 204, 197, 131, 193, 194, 192, 204, 205], 
        [231, 203, 203, 207, 205, 202, 195, 132, 233, 231, 131, 215, 132, 200, 205, 207, 193, 132, 197, 132, 212, 203, 209, 202, 192, 132, 203, 194, 132, 198, 197, 199, 203, 202], 
        [230, 202, 202, 206, 204, 203, 194, 133, 232, 230, 130, 214, 133, 201, 204, 206, 192, 133, 196, 133, 213, 202, 208, 203, 193, 133, 202, 195, 133, 199, 196, 198, 202, 203], 
        [229, 201, 201, 205, 207, 200, 193, 134, 235, 229, 129, 213, 134, 202, 207, 205, 195, 134, 199, 134, 214, 201, 211, 200, 194, 134, 201, 192, 134, 196, 199, 197, 201, 200], 
        [228, 200, 200, 204, 206, 201, 192, 135, 234, 228, 128, 212, 135, 203, 206, 204, 194, 135, 198, 135, 215, 200, 210, 201, 195, 135, 200, 193, 135, 197, 198, 196, 200, 201]
    
     */

    let mut map: HashMap<Vec<u8>, u32> = HashMap::new();
    for v in &vec {
        let score = get_score(v);
        map.insert(v.to_vec(), score);
    }
    

    let min = map.iter()
    .min_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, v)| (k,v))
    .unwrap();

    Some((min.0.clone(), *min.1))  
    
    
}

#[cfg(test)]
mod set1_three_tests{
    use super::*;    
    #[test]
    fn xor_cipher_test(){
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ans_tuple = xor_cipher(input);
        let decoded = String::from_utf8(ans_tuple.unwrap().0.to_vec()).unwrap();
        println!("{}", decoded);
        assert_eq!(decoded, "Cooking MC's like a pound of bacon".to_owned());
    }
}
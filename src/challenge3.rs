use std::{cmp::{min, Ordering}, collections::HashMap};
use itertools::Itertools;
use lazy_static::lazy_static;


lazy_static! {
    static ref FREQUENCIES : HashMap<u8,f32>= HashMap::from([
        ('a'.to_ascii_lowercase() as u8, 0.07743208627550165),
        ('b'.to_ascii_lowercase() as u8, 0.01402241586697527),
        ('c'.to_ascii_lowercase() as u8, 0.02665670667329359),
        ('d'.to_ascii_lowercase() as u8, 0.04920785702311875),
        ('e'.to_ascii_lowercase() as u8, 0.13464518994079883),
        ('f'.to_ascii_lowercase() as u8, 0.025036247121552113),
        ('g'.to_ascii_lowercase() as u8, 0.017007472935972733),
        ('h'.to_ascii_lowercase() as u8, 0.05719839895067157),
        ('i'.to_ascii_lowercase() as u8, 0.06294794236928244),
        ('j'.to_ascii_lowercase() as u8, 0.001267546400727001),
        ('k'.to_ascii_lowercase() as u8, 0.005084890317533608),
        ('l'.to_ascii_lowercase() as u8, 0.03706176274237046),
        ('m'.to_ascii_lowercase() as u8, 0.030277007414117114),
        ('n'.to_ascii_lowercase() as u8, 0.07125316518982316),
        ('o'.to_ascii_lowercase() as u8, 0.07380002176297765),
        ('p'.to_ascii_lowercase() as u8, 0.017513315119093483),
        ('q'.to_ascii_lowercase() as u8, 0.0009499245648139707),
        ('r'.to_ascii_lowercase() as u8, 0.06107162078305546),
        ('s'.to_ascii_lowercase() as u8, 0.061262782073188304),
        ('t'.to_ascii_lowercase() as u8, 0.08760480785349399),
        ('u'.to_ascii_lowercase() as u8, 0.030426995503298266),
        ('v'.to_ascii_lowercase() as u8, 0.01113735085743191),
        ('w'.to_ascii_lowercase() as u8, 0.02168063124398945),
        ('x'.to_ascii_lowercase() as u8, 0.0019880774173815607),
        ('y'.to_ascii_lowercase() as u8, 0.022836421813561863),
        ('z'.to_ascii_lowercase() as u8, 0.0006293617859758195),
    ]);
}

#[derive(Debug)]
pub struct ScoredGuess {
    pub score:f32,
    pub key:Option<u8>,
    pub cipher_text:Option<Vec<u8>>,
    pub plain_text:Option<String>,
}

impl PartialEq for ScoredGuess {
    fn eq(&self, other: &ScoredGuess) -> bool {
        (self.score - other.score) < f32::EPSILON
    }
}

impl PartialOrd for ScoredGuess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredGuess {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ordering) = self.score.partial_cmp(&other.score) {
            ordering
        } else if !self.score.is_nan() {
            Ordering::Less
        } else if !other.score.is_nan() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Eq for ScoredGuess {}

impl ScoredGuess {
    pub fn new() -> Self {
        ScoredGuess {
            score : std::f32::MAX,
            key : None,
            cipher_text : None,
            plain_text : None,
        }
    }    
    pub fn from_key(score:f32, key:u8) -> Self {
        ScoredGuess {
            score : score,
            key : Some(key),
            cipher_text : None,
            plain_text : None,
        }
    }    
}

fn get_character_counts(v: &[u8]) -> HashMap<u8, f32> {
    let mut counts: HashMap<u8, f32> = HashMap::new();
    let len = v.len();
    //for &c in v.iter() {
    for c in 0..=255 {
        //let count = counts.entry(c).or_insert(0f32);
        //*count += 1f32;       
        let count = v.iter().filter(|x| &c == *x).count() as f32;
        counts.entry(c).or_insert(count);
    }
    for (_, val) in counts.iter_mut(){
        *val = *val/len as f32;
    }    

    counts
}

pub fn crack_xor_cipher(bytes :&[u8]) -> ScoredGuess {
    let mut best_guess = ScoredGuess::new();
    let ct_len = bytes.len();
    let ct_freqs = get_character_counts(bytes);
    for candidate_key in 0..=255 {
        let mut score = 0f32;
        //for (letter, expected_freq) in FREQUENCIES.keys().sorted(){
        for key in FREQUENCIES.keys().sorted(){
            let temp = ct_freqs.get(&(key ^ candidate_key)).unwrap();
            score += (FREQUENCIES[key] - temp).abs();
        }
        let guess = ScoredGuess::from_key(score, candidate_key);
        best_guess = min(best_guess, guess);
    }
    if best_guess.key == None {
        panic!("no key found (this should never happen!");
    }
    best_guess.cipher_text = Some(bytes.to_vec());
    let temp : Vec<u8> = vec![best_guess.key.unwrap();ct_len];
    let temp2 = bytes.iter().zip(temp).map(|(b1,b2)| b1^b2).collect::<Vec<_>>();
    let mut str2 = String::new();
    str2.push_str(&String::from_utf8_lossy(&temp2));
    best_guess.plain_text = Some(str2);

    best_guess
}

#[cfg(test)]
mod set1_three_tests{
    use super::*;    
    use crate::challenge1;
    #[test]
    fn xor_cipher_test(){
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let bytes = challenge1::hex_to_bytes(input).unwrap();
        let best_guess = crack_xor_cipher(&bytes);
        //let (score,key,cipher_text,plain_text) = String::from_utf8(ans_tuple.unwrap().0.to_vec()).unwrap();
        let (_score,key,_cipher_text,plain_text) = (best_guess.score, best_guess.key.unwrap(), 
                                        best_guess.cipher_text.unwrap(),best_guess.plain_text.unwrap());
        println!("{}", key);
        println!("{}", plain_text);
        assert_eq!(plain_text, "Cooking MC's like a pound of bacon".to_owned());
    }
}
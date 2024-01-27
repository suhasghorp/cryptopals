
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

#[cfg(test)]
mod set1_six_tests{
    use super::*;  
    
    #[test]
    fn xor_break_test(){
        let x = hamming_distance(b"this is a test", b"wokka wokka!!!");
        println!("{}", x);
    }
}
    


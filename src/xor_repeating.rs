
pub fn xor_repeating(bytes : &[u8], password : &[u8]) -> Vec<u8> {
    password.iter()
    .cycle()
    .zip(bytes)
    .map(|(k,b)| k^b)
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod set1_five_tests{
    use super::*;  
    use crate::hex_base64::to_hex_str;  
    #[test]
    fn xor_repeating_test(){
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let password = b"ICE";
        let output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let xored = xor_repeating(input, password);
        assert_eq!(output, to_hex_str(&xored));
    }
}
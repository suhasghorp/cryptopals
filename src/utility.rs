use base64::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn from_base64_file(path: &Path) -> Vec<u8> {
    let mut content = String::new();
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        content.push_str(line.unwrap().trim());
    }
    BASE64_STANDARD.decode(content).unwrap()
}

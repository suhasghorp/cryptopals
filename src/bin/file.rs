
use std::fs;
use std::env;
use std::io::Error;
use std::collections::HashMap;

fn main() -> Result<(), Error> {

    let mut path = env::current_exe()?;
    path.pop();
    path.push("data");
    path.push("frankenstein.txt");

    let text = fs::read_to_string(path)?;

    let _map = get_freq_lowercase(&text);
    Ok(())
}

fn get_freq_lowercase(text : &str) -> HashMap<char,f32> {
    let mut letter_counts: HashMap<char,f32> = HashMap::new();
    for c in 'a'..='z'{
        let cnt = text.chars().filter(|x| *x == c).count();
        letter_counts.insert(c, cnt as f32);
    };     
    let total:f32 = letter_counts.values().sum();
    letter_counts = letter_counts.into_iter().map(|(key,value)| (key, value/total)).collect();
    letter_counts
}
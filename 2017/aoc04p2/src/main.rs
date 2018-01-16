use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.split("\n");
    let mut good = 0;
    for line in lines {
        if pass_check(line) {
            good += 1;
        }
    }
    println!("{}", good - 1);
}

fn pass_check(pass: &str) -> bool {
    let mut word_map: HashMap<String, bool> = HashMap::new();
    let words = pass
        .split(" ")
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_by(|a, b| a.cmp(b));
            return chars.into_iter().collect::<String>()
        });
    for word in words {
        if word_map.contains_key(&word) {
            return false
        }
        word_map.insert(word, true);
    }
    return true
}
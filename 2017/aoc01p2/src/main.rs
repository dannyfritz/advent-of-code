use std::fs::File;
use std::io::prelude::*;

fn main() {
    let contents = read_contents("./input");
    let digits = chars_to_digits(&contents);
    println!("{}", run_capcha(digits));
}

fn run_capcha(digits: Vec<u32>) -> u32 {
    let mut sum = 0;
    for (i, digit) in digits.iter().enumerate() {
        if digit == digits.get((i + digits.len() / 2) % digits.len()).expect("could not get") {
            sum += digit
        }
    }
    sum
}

fn read_contents(filename: &str) -> String {
    let mut file = File::open(filename).expect("File not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file!");
    return contents;
}

fn chars_to_digits(string: &str) -> Vec<u32> {
    string.chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}
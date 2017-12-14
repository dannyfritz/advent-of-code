use std::fs::File;
use std::io::prelude::*;

fn main() {
    let contents = read_contents("./input");
    let mut sum = 0;
    for row in contents.split('\n') {
        if row.len() <= 0 {
            continue;
        }
        let max: u32 = row.split('\t').map(|s| s.parse().unwrap()).max().unwrap();
        let min: u32 = row.split('\t').map(|s| s.parse().unwrap()).min().unwrap();
        println!("{} - {} = {}", max, min, max - min);
        sum += max - min;
    }
    println!("{}", sum);
}

fn read_contents(filename: &str) -> String {
    let mut file = File::open(filename).expect("File not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file!");
    return contents;
}

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let contents = read_contents("./input");
    let mut sum = 0;
    for row in contents.split('\n') {
        if row.len() <= 0 {
            continue;
        }
        let mut cells = row.split('\t')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>();
        cells.sort_unstable();
        loop {
            match cells.clone().split_first() {
                Some((first, elements)) => {
                    cells = elements.to_vec();
                    match cells.iter().find(|&d| first % d == 0 || d % first == 0) {
                        Some(d) => {
                            sum += std::cmp::max(d, first) / std::cmp::min(d, first);
                            break;
                        },
                        None => continue,
                    }
                },
                None => break,
            }
        }
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

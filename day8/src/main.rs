use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut counts = 0;
    let file = File::open("input.txt").unwrap();
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let (_, second) = line.trim().rsplit_once("|").unwrap();
            String::from(second)
        })
        .for_each(|line| {
            for segment in line.split_whitespace() {
                match segment.len() {
                    2 | 3 | 4 | 7 => counts += 1,
                    _ => (),
                }
            }
        });
    println!("{}", counts);
}

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut start = String::new();
    let _ = BufReader::new(file).read_line(&mut start).unwrap();

    let crabs: Vec<i64> = start
        .trim()
        .split(",")
        .filter_map(|num_str| num_str.parse::<i64>().ok())
        .collect();

    let max_pos = *crabs.iter().max().unwrap();

    let ideal_pos = (0..max_pos)
        .map(|i| fuel_consumption(&crabs, i))
        .min()
        .unwrap();

    println!("{}", ideal_pos);
}

fn fuel_consumption(crabs: &Vec<i64>, pos: i64) -> i64 {
    let mut result = 0;
    for c in crabs {
        result += (c - pos).abs();
    }
    result
}

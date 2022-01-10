use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let pairs: HashMap<char, char> =
        HashMap::from([('(', ')'), ('<', '>'), ('[', ']'), ('{', '}')]);
    let scores: HashMap<char, usize> =
        HashMap::from([(')', 3), ('>', 25137), (']', 57), ('}', 1197)]);
    let mut score = 0;
    let file = File::open("input.txt").unwrap();
    let chars: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|s| s.trim().chars().collect())
        .collect();

    for line in chars {
        let mut state = Vec::new();
        for c in line {
            if pairs.contains_key(&c) {
                state.push(c);
            } else if let Some(closer) = pairs.get(state.last().unwrap()) {
                if *closer == c {
                    state.pop();
                } else {
                    score += scores.get(&c).unwrap();
                    break;
                }
            }
        }
    }

    println!("{}", score);
}

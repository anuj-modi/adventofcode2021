use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::vec::Vec;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn get_numbers() -> Vec<String> {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut numbers = Vec::new();
    for line in lines {
        numbers.push(line.unwrap());
    }
    numbers
}

fn get_popular() -> (String, String) {
    let numbers = get_numbers();
    let mut counts = Vec::new();
    for number in numbers {
        for (i, c) in number.chars().enumerate() {
            if i == counts.len() {
                counts.push((0, 0))
            }

            match c {
                '0' => counts[i].0 += 1,
                '1' => counts[i].1 += 1,
                _ => (),
            }
        }
    }
    let mut popular = String::new();
    let mut unpopular = String::new();
    for (zeros, ones) in counts {
        if zeros < ones {
            popular.push('1');
            unpopular.push('0');
        } else {
            unpopular.push('1');
            popular.push('0');
        }
    }

    (popular, unpopular)
}

fn part1() -> usize {
    let (popular, unpopular) = get_popular();
    let gamma = usize::from_str_radix(popular.as_str(), 2).unwrap();
    let eps = usize::from_str_radix(unpopular.as_str(), 2).unwrap();

    println!("gamma is {}, eps is {}", gamma, eps);

    gamma * eps
}

fn number_filter<'a>(
    ideal: &String,
    idx: usize,
    numbers: impl IntoIterator<Item = &'a String>,
) -> &'a String {
    if idx == ideal.len() {
        numbers.into_iter().nth(0).unwrap()
    } else {
        number_filter(
            ideal,
            idx + 1,
            numbers
                .into_iter()
                .filter(|&s| s.as_bytes()[idx] == (ideal).as_bytes()[idx]),
        )
    }
}

fn part2() -> usize {
    let numbers = get_numbers();
    let (popular, unpopular) = get_popular();

    let o2 = number_filter(&popular, 0, &numbers);
    let co2 = number_filter(&unpopular, 0, &numbers);

    let o2 = usize::from_str_radix(o2.as_str(), 2).unwrap();
    let co2 = usize::from_str_radix(co2.as_str(), 2).unwrap();

    o2 * co2
}

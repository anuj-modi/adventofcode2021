use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    println!("Hello, world!");
    part1();
    part2();
}

fn part1() {
    let numbers = get_depths("input.txt");
    let increases = count_increases(numbers);
    println!("{}", increases);
}

fn count_increases(numbers: Vec<i32>) -> i32 {
    let mut increases = 0;
    for i in 0..numbers.len() - 1 {
        if numbers[i] < numbers[i + 1] {
            increases += 1;
        }
    }
    increases
}

fn get_depths(filepath: &str) -> Vec<i32> {
    let file = File::open(filepath).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            numbers.push(line.parse().unwrap());
        }
    }
    numbers
}

fn sum_windows(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..numbers.len() - 2 {
        result.push(numbers[i..=i + 2].into_iter().sum::<i32>());
    }
    result
}

fn part2() {
    let numbers = get_depths("input.txt");
    let numbers = sum_windows(numbers);

    let increases = count_increases(numbers);
    println!("{}", increases);
}

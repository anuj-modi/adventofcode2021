use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut flashes = 0;
    let file = File::open("input.txt").unwrap();
    let mut numbers: Vec<Vec<i32>> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| -> Vec<i32> {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .filter_map(|i| i32::try_from(i).ok())
                .collect()
        })
        .collect();

    println!("{:?}", numbers);

    step(&mut numbers);

    println!("{}", flashes);
}

fn step(numbers: &mut Vec<Vec<i32>>) {
    numbers
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|n| *n += 1));

    for i in 0..numbers.len() {
        for j in 0..numbers[i].len() {
            if numbers[i][j] == 10 {
                numbers[i][j] = 0;
                if i > 0 {
                    numb
                }
            }
        }
    }
}

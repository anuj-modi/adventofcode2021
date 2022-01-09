use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut risk = 0;
    let file = File::open("input.txt").unwrap();
    let numbers: Vec<Vec<i32>> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| -> Vec<i32> {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .filter_map(|i| i32::try_from(i).ok())
                .collect()
        })
        .collect();

    for i in 0..numbers.len() {
        for j in 0..numbers[i].len() {
            let mut low_point = true;

            // comapre to left
            if j > 0 && numbers[i][j] >= numbers[i][j - 1] {
                low_point = false
            }

            // compare to right
            if j + 1 < numbers[i].len() && numbers[i][j] >= numbers[i][j + 1] {
                low_point = false
            }

            // compare to below
            if i + 1 < numbers.len() && numbers[i][j] >= numbers[i + 1][j] {
                low_point = false
            }

            // compare to above
            if i > 0 && numbers[i][j] >= numbers[i - 1][j] {
                low_point = false
            }

            if low_point {
                // println!("num is {}, i is {}, j is {}", numbers[i][j], i, j);
                risk += numbers[i][j] + 1;
            }
        }
    }
    println!("{}", risk);
}

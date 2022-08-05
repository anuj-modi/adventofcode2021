use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> i32 {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut h = 0;
    let mut v = 0;
    for line in lines {
        let line = line.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        let n: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "forward" => h += n,
            "up" => v -= n,
            "down" => v += n,
            _ => (),
        }
    }

    h * v
}

fn part2() -> i32 {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;
    for line in lines {
        let line = line.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        let n: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "up" => aim -= n,
            "down" => aim += n,
            "forward" => {
                h += n;
                v += n * aim;
            }
            _ => (),
        }
    }

    h * v
}

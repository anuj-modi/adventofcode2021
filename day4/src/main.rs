use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::Iterator;
use std::vec::Vec;

fn main() {
    let mut bingo = BingoGame::new();
    println!("{:?}", bingo.play_game());
}

#[derive(Debug)]
struct BingoGame {
    calls: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn new() -> Self {
        let file = File::open("input.txt").unwrap();
        let mut reader = BufReader::new(file);

        let mut first_line = String::new();
        reader.read_line(&mut first_line).unwrap();
        let calls: Vec<usize> = first_line
            .split(",")
            .filter_map(|num_str| num_str.parse::<usize>().ok())
            .collect();

        let mut rest = String::new();
        reader.read_to_string(&mut rest).unwrap();

        let boards = rest
            .split("\n\n")
            .map(|segment| BingoBoard::new(segment))
            .collect();

        Self {
            calls: calls,
            boards: boards,
        }
    }

    fn play_game(&mut self) -> Option<usize> {
        for &number in &self.calls {
            for board in &mut self.boards {
                board.call_number(number);
                if board.is_complete() {
                    return Some(board.score(number));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<(usize, bool)>>,
}

impl BingoBoard {
    fn new(board: &str) -> Self {
        let mut result = Vec::new();
        for row in board.split("\n") {
            let row: Vec<(usize, bool)> = row
                .split_whitespace()
                .filter_map(|s| match s.parse::<usize>() {
                    Ok(n) => Some((n, false)),
                    _ => None,
                })
                .collect();
            if !row.is_empty() {
                result.push(row)
            }
        }
        Self { board: result }
    }

    fn call_number(&mut self, called_num: usize) {
        for row in self.board.iter_mut() {
            for (number, called) in row.iter_mut() {
                if *number == called_num {
                    *called = true;
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        // if any columns are full
        for r in 0..self.board.len() {
            let mut column_complete = true;
            for c in 0..self.board.len() {
                column_complete &= self.board[c][r].1;
            }
            if column_complete {
                return true;
            }
        }

        // If any of the rows are complete
        self.board
            .iter()
            .any(|row| row.iter().all(|&(_, called)| called))
    }

    fn score(&self, called_num: usize) -> usize {
        self.board
            .iter()
            .flatten()
            .filter(|&(_, called)| !called)
            .map(|&(n, _)| n)
            .sum::<usize>()
            * called_num
    }
}

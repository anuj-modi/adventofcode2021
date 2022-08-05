use std::fs::File;
use std::io::{BufRead, BufReader};

const LIFECYCLE: usize = 8;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut start = String::new();
    let _ = BufReader::new(file).read_line(&mut start).unwrap();

    let mut fish: Vec<LanternFish> = start
        .trim()
        .split(",")
        .filter_map(|num_str| num_str.parse::<usize>().ok())
        .map(|n| LanternFish::new(n))
        .collect();

    for _ in 0..80 {
        let new_fish: Vec<LanternFish> = fish
            .iter_mut()
            .filter_map(|lf| lf.advance_state())
            .collect();
        fish.extend(new_fish);
    }

    println!("After 80 days there are {} fish", fish.len());
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct LanternFish {
    timer: usize,
}

impl LanternFish {
    fn new(timer: usize) -> Self {
        Self { timer: timer }
    }

    fn advance_state(&mut self) -> Option<Self> {
        let result;
        if self.timer == 0 {
            self.timer = LIFECYCLE - 1;
            result = Some(Self { timer: LIFECYCLE })
        } else {
            result = None
        }

        self.timer -= 1;
        result
    }
}

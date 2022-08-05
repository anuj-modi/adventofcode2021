use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut point_counts: HashMap<Point, usize> = HashMap::new();
    let file = File::open("input.txt").unwrap();
    BufReader::new(file)
        .lines()
        .flat_map(|l| Line::new(l.unwrap()).points())
        .for_each(|p| {
            let count = point_counts.entry(p).or_insert(0);
            *count += 1;
        });

    let intersections = point_counts.values().filter(|&&v| v >= 2).count();

    println!("{}", intersections)
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(l: String) -> Self {
        let pieces: Vec<i64> = l
            .split(" -> ")
            .map(|s| s.split(",").filter_map(|s| s.parse().ok()))
            .flatten()
            .collect();
        let mut result = Self {
            start: Point {
                x: pieces[0],
                y: pieces[1],
            },
            end: Point {
                x: pieces[2],
                y: pieces[3],
            },
        };

        if result.start.x > result.end.x {
            let tmp = result.start;
            result.start = result.end;
            result.end = tmp;
        }

        result
    }

    fn points(&self) -> PointGenerator {
        PointGenerator::new(self)
    }
}

struct PointGenerator {
    line: Line,
    slope: Option<i64>,
    current: Option<Point>,
}

impl PointGenerator {
    fn new(line: &Line) -> Self {
        let slope;

        let mut line = line.clone();

        if line.end.x == line.start.x {
            slope = None;
            if line.start.y > line.end.y {
                let tmp = line.start;
                line.start = line.end;
                line.end = tmp;
            }
        } else {
            slope = Some((line.end.y - line.start.y) / (line.end.x - line.start.x));
        }

        Self {
            line: line,
            slope: slope,
            current: Some(line.start),
        }
    }
}

impl Iterator for PointGenerator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current {
            None => None,
            Some(current) => {
                let result = current.clone();
                if *current == self.line.end {
                    self.current = None;
                    Some(result)
                } else {
                    match self.slope {
                        Some(0) => {
                            current.x += 1;
                            current.y += 0;
                        }
                        None => {
                            current.y += 1;
                        }
                        _ => return None,
                    };

                    Some(result)
                }
            }
        }
    }
}

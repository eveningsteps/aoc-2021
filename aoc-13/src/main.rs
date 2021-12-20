use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::{PartialEq, Eq};
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn foldable(&self, line: &Line) -> bool {
        match line.kind {
            Direction::Horizontal => self.y > line.coord,
            Direction::Vertical => self.x > line.coord,
        }
    }

    pub fn folded(&self, line: &Line) -> Self {
        if !self.foldable(line) {
            self.clone()
        } else {
            match line.kind {
                Direction::Horizontal => Self {
                    x: self.x,
                    y: self.y - 2 * (self.y - line.coord).abs(),
                },
                Direction::Vertical => Self {
                    x: self.x - 2 * (self.x - line.coord).abs(),
                    y: self.y,
                }
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Line {
    pub kind: Direction,
    pub coord: i32,
}

fn read_input() -> (HashSet<Point>, Vec<Line>) {
    let points: HashSet<Point> = io::stdin().lock().lines().map_while(|s| {
        match s.unwrap().trim() {
            "" => None,
            anything => {
                let (a, b) = anything.split_once(",").unwrap();
                Some(Point {
                    x: a.parse().unwrap(),
                    y: b.parse().unwrap(),
                })
            }
        }
    }).collect();

    // an empty line inbetween two kinds of input is skipped in ^
    let lines: Vec<Line> = io::stdin().lock().lines().map(|s| {
        let part = s.unwrap().trim().split_ascii_whitespace().rev().next().unwrap().to_owned();
        let (axis, coord) = part.split_once("=").unwrap();
        Line {
            coord: coord.parse().unwrap(),
            kind: match axis {
                "x" => Direction::Vertical,
                "y" => Direction::Horizontal,
                _ => panic!("Unknown axis!"),
            }
        }
    }).collect();
    (points, lines)
}

fn solve(p: &HashSet<Point>, lines: &[Line], iterations: usize) -> HashSet<Point> {
    let mut points = p.clone();
    for i in 0..iterations {
        points = points.iter().map(|point| {
            point.folded(&lines[i])
        }).collect();
    }
    points
}

fn main() {
    let (points, lines) = read_input();
    let answer = solve(&points, &lines, lines.len());  // 1 for the first part
    println!("{}", answer.len());

    /*
        from matplotlib import pyplot

        args = [...]
        pyplot.plot([_[0] for _ in args], [_[1] for _ in args], "ro")
        pyplot.show()
    */
    for p in answer {
        print!("({}, {}), ", p.x, p.y);
    }
}

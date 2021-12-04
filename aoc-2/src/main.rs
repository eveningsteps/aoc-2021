use std::io;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Window size for summation
    #[structopt(short, long)]
    part: usize,
}

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Direction {
    fn parse(s: &String) -> Self {
        let v: Vec<&str> = s.split(' ').collect();
        let i: i32 = v[1].parse().unwrap();
        match v[0] {
            "forward" => Self::Forward(i),
            "down" => Self::Down(i),
            "up" => Self::Up(i),
            _ => panic!("Unmatched token: {}", s)
        }
    }
}

fn read_input() -> Vec<Direction> {
    io::stdin().lock().lines().filter_map(|line| {
        Some(Direction::parse(&line.unwrap()))
    }).collect()
}

fn solve_1(v: &[Direction]) -> i32 {
    let mut h: i32 = 0;
    let mut d: i32 = 0;

    for item in v {
        match item {
            Direction::Forward(i) => { h += i; }
            Direction::Up(i) => { d -= i; }
            Direction::Down(i) => { d += i; }
        }
    }
    h * d
}

fn solve_2(v: &[Direction]) -> i32 {
    let mut aim: i32 = 0;
    let mut h: i32 = 0;
    let mut d: i32 = 0;

    for item in v {
        match item {
            Direction::Forward(i) => {
                h += i;
                d += aim * i;
            }
            Direction::Up(i) => { aim -= i; }
            Direction::Down(i) => { aim += i; }
        }
    }
    h * d
}

fn main() {
    let args = Cli::from_args();

    let v = read_input();
    let answer = match args.part {
        1 => solve_1(&v),
        2 => solve_2(&v),
        _ => panic!("Unknown option {}", args.part)
    };
    println!("{}", answer);
}

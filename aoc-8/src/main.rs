use std::io;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    part: i32,
}

#[derive(Debug)]
struct Data {
    pub signal: Vec<String>,
    pub output: Vec<String>,
}

fn read_input() -> Vec<Data> {
    io::stdin().lock().lines().filter_map(|s: Result<String, _>| -> Option<Data> {
        if let Some((signal, output)) = s.unwrap().trim().split_once(" | ") {
            Some(Data {
                signal: signal.split_ascii_whitespace().map(|s| {
                    s.to_string()
                }).collect(),
                output: output.split_ascii_whitespace().map(|s| {
                    s.to_string()
                }).collect(),
            })
        } else {
            None
        }
    }).collect()
}

fn solve(v: &Vec<Data>) -> i32 {
    v.iter().map(|d| -> i32 {
        d.output.iter().map(|o| -> i32 {
            match o.len() {
                // segment count for 1 | 7 | 4 | 8
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        }).sum()
    }).sum()
}

fn solve2(v: &Vec<Data>) -> i32 {
    0
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = if args.part == 1 {
        solve(&input)
    } else {
        solve2(&input)
    };
    println!("{}", answer);
}

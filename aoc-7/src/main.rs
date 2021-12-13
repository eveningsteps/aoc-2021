use std::io;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    part: i32,
}

fn read_input() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(",").map(|s| -> i32 {
        s.parse().unwrap()
    }).collect()
}

fn solve(v: &Vec<i32>) -> i32 {
    let a = v.iter().min().unwrap().to_owned();
    let b = v.iter().max().unwrap().to_owned();

    (a..=b).map(|pos| -> i32 {
        v.iter().map(|x| -> i32 {
            (x - pos).abs()
        }).sum()
    }).min().unwrap()
}

fn dist(a: &i32, b: &i32) -> i32 {
    (a - b).abs() * ((a - b).abs() + 1) / 2
}

fn solve2(v: &Vec<i32>) -> i32 {
    let a = v.iter().min().unwrap().to_owned();
    let b = v.iter().max().unwrap().to_owned();

    (a..=b).map(|pos| -> i32 {
        v.iter().map(|x| -> i32 {
            dist(&pos, x)
        }).sum()
    }).min().unwrap()
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = if args.part == 1 { solve(&input) } else { solve2(&input) };
    println!("{}", answer);
}
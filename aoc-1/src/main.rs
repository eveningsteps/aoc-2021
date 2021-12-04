use std::io;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Window size for summation
    #[structopt(short, long)]
    window_width: usize,
}

fn read_input() -> Vec<i32> {
    io::stdin().lock().lines().filter_map(|line| {
        line.unwrap().parse::<i32>().ok()
    }).collect()
}

fn count(v: &[i32], window_width: usize) -> i32 {
    let mut prev_sum: i32 = 0;
    for i in 0..window_width {
        prev_sum += v[i];
    }
    let mut count = 0;
    for i in window_width..v.len() {
        let curr_sum = prev_sum - v[i - window_width] + v[i];
        if curr_sum > prev_sum {
            count += 1;
        }
    }
    count
}

fn main() {
    let args = Cli::from_args();

    let v = read_input();
    let answer = count(&v, args.window_width);
    println!("{}", answer);
}

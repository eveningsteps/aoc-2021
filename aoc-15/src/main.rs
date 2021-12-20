use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, short)]
    part: i32
}

fn read_input() -> Vec<Vec<i32>> {
    io::stdin().lock().lines().map(|s| {
        s.unwrap().trim().chars().map(|ch| {
            ch.to_digit(10).unwrap().try_into().unwrap()
        }).collect()
    }).collect()
}

fn solve(v: &[Vec<i32>]) -> i32 {
    let mut cost = vec![vec![i32::MAX; v[0].len()]; v.len()];
    cost[0][0] = 0;

    // https://en.wikipedia.org/wiki/Dijkstra's_algorithm
    let mut q: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    q.push((cost[0][0], 0, 0));

    let d = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    while !q.is_empty() {
        let (cur_cost, cur_x, cur_y) = q.pop().unwrap();
        for (dx, dy) in d {
            let next_x = cur_x + dx;
            let next_y = cur_y + dy;
            if !(0 <= next_x && next_x < v[0].len() as i32 && 0 <= next_y && next_y < v.len() as i32) {
                continue;
            }

            let next_dist = -cur_cost + v[next_y as usize][next_x as usize];
            if next_dist < cost[next_y as usize][next_x as usize] {
                cost[next_y as usize][next_x as usize] = next_dist;
                q.push((-next_dist, next_x, next_y));
            }
        }
    }
    *cost.last().unwrap().last().unwrap()
}

fn clamp(x: i32) -> i32 {
    if x < 10 {
        x
    } else {
        x - 9
    }
}

fn scale_map(v: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let my = v.len();
    let mx = v[0].len();
    let mut mp = vec![vec![0; mx * 5]; my * 5];
    for row in 0..my {
        for col in 0..mx {
            for ax in 0..5 {
                for ay in 0..5 {
                    mp[ay * my + row][ax * mx + col] = clamp(v[row][col] + ax as i32 + ay as i32)
                }
            }
        }
    }
    mp
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = if args.part == 1 {
        solve(&input)
    } else {
        solve(&scale_map(&input))
    };
    println!("{}", answer);
}

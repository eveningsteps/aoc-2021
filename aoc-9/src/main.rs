use std::io;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    part: i32,
}

fn read_input() -> Vec<Vec<i32>> {
    io::stdin().lock().lines().map(|s: Result<String, _>| -> Vec<i32> {
        s.unwrap().chars().map(|ch| -> i32 {
            ch.to_digit(10).unwrap().try_into().unwrap()
        }).collect()
    }).collect()
}

fn inbound(v: &[Vec<i32>], r: i32, c: i32) -> bool {
    (0 <= r && r < v.len() as i32) &&
    (0 <= c && c < v[r as usize].len() as i32)
}

fn solve(v: &[Vec<i32>]) -> (Vec<(usize, usize)>, i32) {
    let mut sum = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();

    let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let checker = |r: usize, c: usize| -> bool {
        for (dr, dc) in offsets {
            let near_r = r as i32 + dr;
            let near_c = c as i32 + dc;
            if inbound(v, near_r, near_c) {
                if v[r][c] >= v[near_r as usize][near_c as usize] {
                    return false;
                }
            }
        }
        return true;
    };

    for row in 0..v.len() {
        for col in 0..v[row].len() {
            if checker(row, col) {
                sum += v[row][col] + 1;
                low_points.push((row, col));
            }
        }
    }
    (low_points, sum)
}

fn bfs(v: &[Vec<i32>], visited: &mut [Vec<bool>], r: i32, c: i32) -> i64 {
    if !inbound(v, r, c) {
        return 0;
    }

    let ur = r as usize;
    let uc = c as usize;

    if v[ur][uc] == 9 {
        visited[ur][uc] = true;
    }
    if visited[ur][uc] {
        return 0;
    }

    visited[r as usize][c as usize] = true;
    let mut area = 1;
    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        area += bfs(v, visited, r + dr, c + dc);
    }
    return area;
}

fn solve2(v: &[Vec<i32>], low_points: &[(usize, usize)]) -> i64 {
    let mut visited = vec![vec![false; v[0].len()]; v.len()];
    let mut areas: Vec<i64> = Vec::new();
    for (r, c) in low_points {
        areas.push(bfs(v, &mut visited, *r as i32, *c as i32));
    }
    areas.sort_unstable_by(|a, b| b.cmp(a));
    areas[0] * areas[1] * areas[2]
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let (low_points, sum) = solve(&input);
    if args.part == 1 {
        println!("{}", sum);
    } else {
        let product = solve2(&input, &low_points);
        println!("{}", product);
    }
}

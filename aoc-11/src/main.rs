use std::io;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    part: i32,
}

enum Mode {
    CountFlashes,
    CountSteps,
}

fn read_input() -> Vec<Vec<i32>> {
    io::stdin().lock().lines().map(|s| {
        s.unwrap().trim().chars().map(|ch| {
            ch.to_digit(10).unwrap().try_into().unwrap()
        }).collect()
    }).collect()
}

fn inbound(v: &[Vec<i32>], r: i32, c: i32) -> bool {
    (0 <= r && r < v.len() as i32) &&
    (0 <= c && c < v[r as usize].len() as i32)
}

fn try_distribute_energy(v: &mut [Vec<i32>], r: i32, c: i32) {
    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }

            let rr = r + dr;
            let cc = c + dc;
            if inbound(v, rr, cc) {
                v[rr as usize][cc as usize] = 10.min(v[rr as usize][cc as usize] + 1);
            }
        }
    }
}

fn simulate(v: &[Vec<i32>], steps: i32, mode: Mode) -> i32 {
    let mut octopuses = v.to_owned();
    let mut sum = 0;

    let limit = match mode {
        Mode::CountSteps => i32::MAX,
        Mode::CountFlashes => steps,
    };
    for i in 0..limit {
        let mut state = octopuses.to_owned();
        let mut energy_distributed = vec![vec![false; v[0].len()]; v.len()];

        // increment energy
        for row in 0..state.len() {
            for col in 0..state[row].len() {
                state[row][col] += 1;
            }
        }

        // simulate chain energy distribution
        let mut keep_checking: bool = true;
        while keep_checking {
            keep_checking = false;
            for row in 0..state.len() {
                for col in 0..state[row].len() {
                    if energy_distributed[row][col] {
                        continue;
                    }
                    if state[row][col] == 10 {
                        try_distribute_energy(&mut state, row as i32, col as i32);
                        energy_distributed[row][col] = true;
                        keep_checking = true;
                    }
                }
            }
        }

        if matches!(mode, Mode::CountSteps) {
            let mut all_flashed = true;
            for row in 0..state.len() {
                for col in 0..state[row].len() {
                    all_flashed &= energy_distributed[row][col];
                }
            }
            if all_flashed {
                return i + 1;
            }
        }

        // count flashes
        for row in 0..state.len() {
            for col in 0..state[row].len() {
                if state[row][col] == 10 {
                    state[row][col] = 0;
                    sum += 1;
                }
            }
        }
        octopuses = state;
    }
    sum
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let mode = match args.part {
        1 => Mode::CountFlashes,
        2 => Mode::CountSteps,
        _ => panic!("unknown mode")
    };
    let answer = simulate(&input, 100, mode);
    println!("{}", answer);
}

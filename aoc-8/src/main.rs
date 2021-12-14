use std::io;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    part: i32,
}

type Digit = HashSet<char>;

#[derive(Debug)]
struct Data {
    pub signal: Vec<Digit>,
    pub output: Vec<Digit>,
}

impl Data {
    fn find_by_len(v: &[Digit], l: usize) -> usize {
        (0..v.len()).filter(|idx| {
            v[*idx].len() == l
        }).next().unwrap()
    }

    fn all_by_len(v: &[Digit], l: usize) -> Vec<usize> {
        (0..v.len()).filter(|idx| {
            v[*idx].len() == l
        }).collect()
    }

    fn overlap(a: &Digit, b: &Digit) -> HashSet<char> {
        a & b
    }

    fn find_by_overlap(
        v: &[Digit],
        base: usize,
        target_len: usize,
        num_overlaps: usize
    ) -> usize {
        let base_idx = Self::find(v, base);
        let digits_idx = Self::all_by_len(v, target_len);
        digits_idx.iter().filter(|idx| {
            Self::overlap(&v[base_idx], &v[**idx]).len() == num_overlaps
        }).next().unwrap().to_owned()
    }

    pub fn find(v: &[Digit], digit: usize) -> usize {
        match digit {
            // unique segment count
            1 => Self::find_by_len(v, 2),
            4 => Self::find_by_len(v, 4),
            7 => Self::find_by_len(v, 3),
            8 => Self::find_by_len(v, 7),

            // may be found by unique overlaps
            3 => Self::find_by_overlap(v, 1, 5, 2),
            6 => Self::find_by_overlap(v, 7, 6, 2),
            9 => Self::find_by_overlap(v, 4, 6, 4),

            // unique overlaps with other previously found digits
            5 => Self::find_by_overlap(v, 6, 5, 5),
            2 => Self::find_by_overlap(v, 5, 5, 3),

            // the only remaining digit
            0 => {
                let mut all_idx: HashSet<usize> = (0..10).collect();
                for i in 1..=9 {
                    all_idx.remove(&Self::find(v, i));
                }
                all_idx.iter().next().unwrap().to_owned()
            },
            _ => panic!("Not a digit lol"),
        }
    }
}

fn read_input() -> Vec<Data> {
    io::stdin().lock().lines().filter_map(|s: Result<String, _>| -> Option<Data> {
        if let Some((signal, output)) = s.unwrap().trim().split_once(" | ") {
            Some(Data {
                signal: signal.split_ascii_whitespace().map(|s| {
                    s.chars().collect()
                }).collect(),
                output: output.split_ascii_whitespace().map(|s| {
                    s.chars().collect()
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
    let mut sum = 0;
    for data in v {
        let mut mapping: HashMap<usize, Digit> = HashMap::new();
        for value in 0..=9 {
            let idx = Data::find(&data.signal, value);
            mapping.insert(value, data.signal[idx].clone());
        }

        let lookup = |digit| -> i32 {
            for (k, v) in mapping.iter() {
                if v == digit {
                    return k.to_owned() as i32;
                }
            }
            panic!("Not found -- impossible");
        };

        let mut intermediate = 0;
        for value in &data.output {
            intermediate *= 10;
            intermediate += lookup(value);
        }
        sum += intermediate;
    }
    sum
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

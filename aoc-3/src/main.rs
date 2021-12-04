use std::io;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Iteratively remove words that don't pass the check
    #[structopt(short, long)]
    reduce: bool,
}

struct Bit {
    value: char,
    count: i32,
}

impl Bit {
    fn new(i: char) -> Self {
        Bit {
            value: i,
            count: 0,
        }
    }
}

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().map(|l| {
        l.unwrap()
    }).collect()
}

fn count_items<T>(v: &Vec<String>, cmp_and_choose: T) -> Vec<i32>
where
    T: Fn(Bit, Bit) -> char,
{
    let mut word = Vec::with_capacity(v[0].len());

    for i in 0..v[0].len() {
        let mut b0 = Bit::new('0');
        let mut b1 = Bit::new('1');
        for j in 0..v.len() {
            match v[j].as_bytes()[i] as char {
                '0' => { b0.count += 1 }
                '1' => { b1.count += 1 }
                _ => {}
            }
        }
        word.push(cmp_and_choose(b0, b1));
    }
    word.iter().map(|c| {
        c.to_digit(10).unwrap() as i32
    }).collect()
}

fn reduce_items<T>(v: &Vec<String>, cmp_and_choose: T) -> Vec<i32>
where
    T: Fn(Bit, Bit) -> char,
{
    let mut words = v.clone();
    for i in 0..words[0].len() {
        let mut b0 = Bit::new('0');
        let mut b1 = Bit::new('1');
        for j in 0..words.len() {
            match words[j].as_bytes()[i] as char {
                '0' => { b0.count += 1 }
                '1' => { b1.count += 1 }
                _ => {}
            }
        }
        let winner = cmp_and_choose(b0, b1);
        words = words.into_iter().filter(|w| {
            w.as_bytes()[i] as char == winner
        }).collect();
        if words.len() == 1 {
            break
        }
    }
    words[0].chars().map(|c| {
        c.to_digit(10).unwrap() as i32
    }).collect()
}

fn to_decimal(v: &Vec<i32>) -> i32 {
    let mut value = 0;
    for i in v {
        value *= 2;
        value += i;
    }
    value
}

fn solve(v: &mut Vec<String>, reduce: bool) -> i32 {
    let max_ = |a: Bit, b: Bit| {
        if a.count == b.count {
            '1'
        } else if a.count > b.count { a.value } else { b.value }
    };
    let min_ = |a: Bit, b: Bit| {
        if a.count == b.count {
            '0'
        } else if a.count < b.count { a.value } else { b.value }
    };
    let gamma = if reduce { reduce_items(v, max_) } else { count_items(v, max_) };
    let epsilon = if reduce { reduce_items(v, min_) } else { count_items(v, min_) };

    to_decimal(&gamma) * to_decimal(&epsilon)
}

fn main() {
    let args = Cli::from_args();

    let mut v = read_input();
    let answer = solve(&mut v, args.reduce);
    println!("{}", answer);
}

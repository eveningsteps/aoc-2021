use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use counter::Counter;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, short)]
    steps: i32,
}

type Rules = HashMap<[char; 2], char>;

fn read_input() -> (String, Rules) {
    let mut template = String::new();
    io::stdin().read_line(&mut template).unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut rules: Rules = HashMap::new();
    for l in io::stdin().lock().lines() {
        let line = l.unwrap();
        let parts = line.trim().split_once(" -> ").unwrap();

        let from: Vec<char> = parts.0.chars().collect();
        let to = parts.1.chars().next().unwrap();
        rules.insert([from[0], from[1]], to);
    }

    (template, rules)
}

fn solve(t: &String, r: &Rules, steps: i32) -> i64 {
    let trimmed = t.trim();
    let mut chars: Counter<char, i64> = trimmed.chars().collect();
    let mut digraphs: Counter<[char; 2], i64> = Counter::new();

    let mut iter = trimmed.chars();
    let mut prev_char = iter.next().unwrap();
    for ch in iter {
        digraphs[&[prev_char, ch]] += 1;
        prev_char = ch;
    }

    for _ in 0..steps {
        let mut new_digraphs: Counter<[char; 2], i64> = digraphs.clone();
        for (digraph, digraph_cnt) in &digraphs {
            if let Some(new_char) = r.get(digraph) {
                chars[new_char] += digraph_cnt;
                new_digraphs[digraph] -= digraph_cnt;
                new_digraphs[&[digraph[0], *new_char]] += digraph_cnt;
                new_digraphs[&[*new_char, digraph[1]]] += digraph_cnt;
            }
        }
        digraphs = new_digraphs;
    }


    let freqs = chars.most_common();
    freqs.first().unwrap().1 - freqs.last().unwrap().1
}

fn main() {
    let args = Cli::from_args();
    let (template, rules) = read_input();
    let answer = solve(&template, &rules, args.steps);
    println!("{}", answer);
}

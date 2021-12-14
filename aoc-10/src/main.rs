use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, short)]
    part: i32
}

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().filter_map(|s| {
        let trimmed = s.unwrap().trim().to_owned();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    }).collect()
}

enum CheckResult {
    ValidSequence,
    InvalidSequence(char),
    IncompleteSequence(Vec<char>)
}

fn check_sequence(matching: &HashMap<char, char>, s: &String) -> CheckResult {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if stack.is_empty() || *stack.last().unwrap() != matching[&ch] {
                    return CheckResult::InvalidSequence(ch);
                } else {
                    stack.pop();
                }
            }
            something_else => panic!("Unexpected character {:?}", something_else)
        }
    }
    if stack.is_empty() {
        return CheckResult::ValidSequence;
    } else {
        return CheckResult::IncompleteSequence(stack);
    }
}

fn solve(v: &[String], complete: bool) -> i64 {
    let scores: HashMap<char, i64> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter().collect();
    let matching: HashMap<char, char> = [
        (')', '('), (']', '['), ('}', '{'), ('>', '<'),
        ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>'),
    ].into_iter().collect();
    let complete_scores: HashMap<char, i64> = [(')', 1), (']', 2), ('}', 3), ('>', 4)].into_iter().collect();

    let mut to_complete: Vec<Vec<char>> = Vec::new();
    let invalid_score = v.iter().map(|s| -> i64 {
        match check_sequence(&matching, s) {
            CheckResult::ValidSequence => 0,
            CheckResult::InvalidSequence(ch) => {
                scores[&ch]
            },
            CheckResult::IncompleteSequence(stack) => {
                to_complete.push(stack);
                0
            },
        }
    }).sum();

    if !complete {
        invalid_score
    } else {
        let mut completeness_scores: Vec<i64> = to_complete.iter().map(|st| {
            let mut score: i64 = 0;
            for ch in st.iter().rev() {
                score *= 5;
                score += complete_scores[&matching[ch]];
            }
            score
        }).collect();
        completeness_scores.sort();
        completeness_scores[completeness_scores.len() / 2]
    }
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = solve(&input, args.part == 2);
    println!("{}", answer);
}

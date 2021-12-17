use std::collections::{HashMap,HashSet};
use std::io;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, short)]
    part: i32,
}

const START: &str = "start";
const END: &str = "end";

type Graph = HashMap<String, Vec<String>>;

fn read_input() -> Graph {
    let mut map: Graph = HashMap::new();
    for line in io::stdin().lock().lines() {
        if let Some((from, to)) = line.unwrap().trim().split_once("-") {
            map.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
            map.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
        }
    }
    map
}

fn is_reentrable(s: &String) -> bool {
    s.chars().next().unwrap().is_ascii_uppercase()
}

fn dfs(m: &Graph, v: &HashSet<String>, node: &String, golden_node_used: bool) -> i32 {
    let mut visited = v.clone();
    if !is_reentrable(node) {
        visited.insert(node.to_owned());
    }

    if node == END {
        return 1;
    }

    let mut local_sum = 0;
    for adjacent_node in m.get(node).unwrap().iter() {
        if adjacent_node == START {
            continue;
        }

        let adjacent_visited = visited.contains(adjacent_node);
        if adjacent_visited {
            if golden_node_used {
                continue;
            } else {
                // give the second chance
                let mut visited_but_with_second_chance = visited.clone();
                visited_but_with_second_chance.remove(adjacent_node);
                local_sum += dfs(m, &visited_but_with_second_chance, adjacent_node, true);
            }
        } else {
            local_sum += dfs(m, &visited, adjacent_node, golden_node_used);
        }
    }
    return local_sum;
}

fn solve(map: &Graph, use_single_reentrable_small_cave: bool) -> i32 {
    let visited = HashSet::new();
    let start = START.to_string();
    dfs(map, &visited, &start, use_single_reentrable_small_cave)
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = solve(&input, args.part == 1);
    println!("{}", answer);
}

use std::collections::{HashMap,HashSet};
use std::io;
use std::io::prelude::*;

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

fn dfs(m: &Graph, v: &HashSet<String>, node: &String) -> i32 {
    let mut visited = v.clone();
    if !node.chars().next().unwrap().is_ascii_uppercase() {
        visited.insert(node.to_owned());
    }

    if node == END {
        return 1;
    }

    let mut local_sum = 0;
    for adjacent_node in m.get(node).unwrap().iter() {
        if visited.contains(adjacent_node) {
            continue;
        }
        local_sum += dfs(m, &visited, adjacent_node);
    }
    return local_sum;
}

fn solve(map: &Graph) -> i32 {
    let visited = HashSet::new();
    let start = START.to_string();
    dfs(map, &visited, &start)
}

fn main() {
    let input = read_input();
    let answer = solve(&input);
    println!("{}", answer);
}

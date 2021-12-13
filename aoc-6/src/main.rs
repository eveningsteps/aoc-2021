use std::io;
use std::fmt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// How many days to simulate
    #[structopt(short, long)]
    days: i32,
}

#[derive(Debug, Clone)]
struct Lanternfish {
    pub age: i32
}

impl fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.age)
    }
}

impl Lanternfish {
    pub fn new() -> Self {
        Self {
            age: 8
        }
    }

    pub fn reset(&mut self) {
        self.age = 6;
    }

    pub fn simulate(&mut self) -> Option<Self> {
        match self.age {
            0 => {
                self.reset();
                Some(Self::new())
            }
            _ => {
                self.age -= 1;
                None
            }
        }
    }
}

fn read_input() -> Vec<Lanternfish> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(",").map(|s| {
        let age: i32 = s.parse().unwrap();
        Lanternfish { age }
    }).collect()
}

fn simulate(v: &Vec<Lanternfish>, days: i32) -> i32 {
    let mut fish = v.to_vec();
    for d in 0..days {
        // println!("Simulating day {}", d);
        let mut new_fish = Vec::<Lanternfish>::new();
        for f in fish.iter_mut() {
            if let Some(l) = (*f).simulate() {
                new_fish.push(l);
            }
        }
        fish.append(&mut new_fish);
        // println!("Day {}: {}", d, fish.iter().map(|f| -> String {
        //     f.to_string()
        // }).collect::<Vec<String>>().join(","));
    }
    fish.len() as i32
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = simulate(&input, args.days);
    println!("{}", answer);
}

use std::io;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// How many days to simulate
    #[structopt(short, long)]
    days: i32,
}

fn read_input() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(",").map(|s| -> i32 {
        s.parse().unwrap()
    }).collect()
}


fn dynamic(v: &Vec<i32>, days: i32) -> i64 {
    let mut fish: Vec<Vec<i64>> = Vec::new();
    let max_fish_lifetime = 8;
    for _ in 0..days {
        fish.push(vec![0; max_fish_lifetime + 1]);
    }
    for f in v {
        fish[0][*f as usize] += 1;
    }

    for day in 1..(days as usize) {
        for i in (0..max_fish_lifetime).rev() {
            fish[day][i] = fish[day - 1][i + 1];
        }
        fish[day][max_fish_lifetime] += fish[day - 1][0];
        fish[day][6] += fish[day - 1][0];
    }
    fish.last().unwrap().iter().sum()
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = dynamic(&input, args.days + 1);
    println!("{}", answer);
}

use std::collections::HashMap;
use std::io;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Which board we are looking for
    #[structopt(short, long)]
    part: i32,
}

#[derive(Debug)]
struct Cell {
    pub num: i32,
    pub marked: bool,
}

impl Cell {
    pub fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Default, Debug)]
struct Board {
    sz: usize,
    num: Vec<Vec<Cell>>,
    loc: HashMap<i32, (usize, usize)>,
}

impl Board {
    pub fn new(sz: usize) -> Self {
        Self {
            sz,
            ..Default::default()
        }
    }

    pub fn add_row(&mut self, row: &Vec<i32>) -> Result<(), ()> {
        let row_idx = self.num.len();
        if row_idx >= self.sz {
            Err(())
        } else {
            let mut buf = vec![];
            for i in 0..self.sz {
                self.loc.insert(row[i], (row_idx, i));
                buf.push(Cell { num: row[i], marked: false });
            }
            self.num.push(buf);
            Ok(())
        }
    }

    pub fn add(&mut self, n: &i32) {
        if let Some((row, col)) = self.loc.get(n) {
            self.num[*row][*col].mark();
        }
    }

    pub fn check_winning_condition(&self) -> bool {
        for i in 0..self.sz {
            let mut marked_in_row = 0;
            let mut marked_in_col = 0;

            for j in 0..self.sz {
                if self.num[i][j].marked {
                    marked_in_col += 1;
                }
                if self.num[j][i].marked {
                    marked_in_row += 1;
                }
            }
            if marked_in_row == self.sz || marked_in_col == self.sz {
                return true;
            }
        }
        false
    }

    pub fn score(&self, n: &i32) -> i32 {
        let mut s = 0;
        for i in 0..self.sz {
            for j in 0..self.sz {
                if !self.num[i][j].marked {
                    s += self.num[i][j].num;
                }
            }
        }
        s * n
    }
}

fn read_input() -> (Vec<i32>, Vec<Board>) {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let numbers: Vec<i32> = buf.trim().split(',').map(|ch| -> i32 {
        ch.parse().unwrap()
    }).collect();

    // skip empty line
    stdin.read_line(&mut buf).unwrap();

    let mut boards: Vec<Board> = Vec::new();
    'input: loop {
        let mut b = Board::new(5);
        for _ in 0..5 {
            buf.clear();
            match stdin.read_line(&mut buf) {
                Ok(_) => {
                    if buf.trim().is_empty() {
                        break 'input;
                    }
                    let row: Vec<i32> = buf.trim().split_ascii_whitespace().map(|i| -> i32 {
                        i.parse().unwrap()
                    }).collect();
                    b.add_row(&row).unwrap();
                }
                Err(_) => break 'input
            }
        }

        boards.push(b);
        // skip empty line, again
        stdin.read_line(&mut buf).unwrap();    
    }
    (numbers, boards)
}

fn solve(numbers: &Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for i in numbers {
        for b in boards.iter_mut() {
            b.add(i);
            if b.check_winning_condition() {
                return b.score(i);
            }
        }
    }
    0
}

fn solve2(numbers: &Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    let mut winners = vec![];
    for i in numbers {
        for (board_num, b) in boards.iter_mut().enumerate().filter(|b| -> bool {
            !b.1.check_winning_condition()
        }) {
            b.add(i);
            if b.check_winning_condition() {
                winners.push((board_num, i));
            }
        }
    }
    if let Some((board_idx, n)) = winners.last() {
        boards[*board_idx].score(n)
    } else {
        0
    }
}

fn main() {
    let args = Cli::from_args();
    let (numbers, mut boards) = read_input();
    let answer = if args.part == 1 {
        solve(&numbers, &mut boards)
    } else {
        solve2(&numbers, &mut boards)
    };
    println!("{}", answer);
}

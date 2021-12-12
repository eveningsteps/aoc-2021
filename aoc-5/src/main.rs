use std::io;
use std::io::prelude::*;
use std::iter::{Iterator, IntoIterator};

#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Line {
    pub a: Point,
    pub b: Point,
}

struct LineIter<'a> {
    line: &'a Line,

    dx: i32,
    dy: i32,

    current_x: i32,
    current_y: i32,
}

fn between(a: i32, x: i32, b: i32) -> bool {
    (a <= x && x <= b) || (b <= x && x <= a)
}

fn pick_gradient(a: &Point, b: &Point) -> (i32, i32) {
    if a.x == b.x {
        (0, 1)
    } else if a.y == b.y {
        (1, 0)
    } else {
        (
            if a.x < b.x { 1 } else { -1 },
            if a.y < b.y { 1 } else { -1 },
        )
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.is_diagonal() {
            None  // XXX: this branch is ignored for the first part ("no diagonal lines")
        } else {
            let p = Self::Item {
                x: self.line.a.x + self.current_x,
                y: self.line.a.y + self.current_y,
            };
            if !between(self.line.a.x, p.x, self.line.b.x) || !between(self.line.a.y, p.y, self.line.b.y) {
                None
            } else {
                self.current_x += self.dx;
                self.current_y += self.dy;
                Some(p)
            }
        }
    }
}

impl Line {
    pub fn from_points(p1: Point, p2: Point) -> Self {
        if p1.x > p2.x || p1.y > p2.y {
            Self { a: p2, b: p1 }
        } else {
            Self { a: p1, b: p2 }
        }
    }

    pub fn is_diagonal(&self) -> bool {
        !(self.a.x == self.b.x || self.a.y == self.b.y)
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = <LineIter<'a> as Iterator>::Item;
    type IntoIter = LineIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let (dx, dy) = pick_gradient(&self.a, &self.b);
        Self::IntoIter {
            line: self,
            current_x: 0,
            current_y: 0,
            dx,
            dy,
        }
    }
}

fn read_input() -> Vec<Line> {
    io::stdin().lock().lines().filter_map(|s| -> Option<Line> {
        match s.unwrap().trim() {
            "" => None,
            l => {
                let points: Vec<Point> = l.split(" -> ").map(|part| -> Point {
                    let coord: Vec<i32> = part.trim().split(",").map(|i| -> i32 {
                        i.parse().unwrap()
                    }).collect();
                    Point {
                        x: coord[0],
                        y: coord[1],
                    }
                }).collect();
                Some(Line::from_points(points[0], points[1]))
            }
        }
    }).collect()
}

fn solve(v: &Vec<Line>) -> i32 {
    let mut h = std::collections::HashMap::<(i32, i32), i32>::new();
    for line in v.iter() {
        for point in line {
            *h.entry((point.x, point.y)).or_insert(0) += 1;
        }
    }
    h.values().map(|v| -> i32 {
        match v {
            0 | 1 => 0,
            _ => 1,
        }
    }).sum()
}

fn main() {
    let input = read_input();
    let answer = solve(&input);
    println!("{}", answer);
}

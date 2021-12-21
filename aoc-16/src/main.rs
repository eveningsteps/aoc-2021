use std::io;
use std::slice;
use std::cmp;
use std::collections::VecDeque;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, short)]
    part: i32
}

type Bit = u8;
type BitStream<'a> = slice::Iter<'a, Bit>;

#[derive(Debug, cmp::Eq, cmp::PartialEq, Clone)]
struct Packet {
    pub version: u8,
    pub r#type: u8,
    pub value: Option<u64>,
    pub children: Vec<Packet>,
}

impl Packet {
    fn empty(version: u8, r#type: u8) -> Self {
        Self {
            version,
            r#type,
            value: None,
            children: Vec::new(),
        }
    }

    fn value(&self) -> u64 {
        match self.r#type {
            0 => {
                let mut sum = 0;
                for ch in &self.children {
                    sum += ch.value();
                }
                sum
            }
            1 => {
                let mut mp = 1;
                for ch in &self.children {
                    mp *= ch.value();
                }
                mp
            }
            2 | 3 => {
                let map = self.children.iter().map(|ch| ch.value());
                match self.r#type {
                    2 => map.min().unwrap(),
                    3 => map.max().unwrap(),
                    _ => panic!("value already handled")
                }
            }
            4 => {
                self.value.unwrap() as u64
            }
            5 | 6 | 7 => {
                let ch = &self.children;
                match self.r#type {
                    5 => if ch[0].value() >  ch[1].value() { 1 } else { 0 }
                    6 => if ch[0].value() <  ch[1].value() { 1 } else { 0 }
                    7 => if ch[0].value() == ch[1].value() { 1 } else { 0 }
                    _ => panic!("value already handled")
                }
            }
            _ => panic!("impossible type value")
        }
    }
}

fn read_input() -> Vec<Bit> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    to_bits(&buf)
}

fn to_bits(buf: &str) -> Vec<Bit> {
    let mut stream: Vec<Bit> = Vec::new();
    for ch in buf.trim().chars() {
        let d = ch.to_digit(16).unwrap();
        for n in (0..4).rev() {
            stream.push(nth_bit(d, n));
        }
    }
    stream
}

fn nth_bit(num: u32, n: i32) -> Bit {
    ((num & (1 << n)) >> n) as Bit
}

fn read_n(s: &mut BitStream, n: i32) -> Vec<Bit> {
    let mut v: Vec<Bit> = Vec::new();
    for _ in 0..n {
        v.push(*s.next().unwrap());
    }
    v
}

// most significant bit goes first
fn to_num(v: &[Bit]) -> u64 {
    let mut acc = 0;
    for i in 0..v.len() {
        acc = (acc << 1) + (v[i] as u64);
    }
    acc
}

fn read_literal(s: &mut BitStream) -> (u32, u64) {
    let mut read = 0;
    let mut bits: Vec<Bit> = Vec::new();
    loop {
        let part = read_n(s, 5);
        read += 5;
        bits.extend(&part[1..5]);
        if part[0] == 1 {
            continue;
        } else {
            break;
        }
    }
    (read, to_num(&mut bits))
}

fn read_children_n(it: &mut BitStream) -> (u32, Vec<Packet>) {
    let mut read = 0;
    let subpacket_count = to_num(&read_n(it, 11));
    read += 11;

    let mut subpackets: Vec<Packet> = Vec::new();
    for _ in 0..subpacket_count {
        let (n, subpacket) = parse_packet(it);
        read += n;
        subpackets.push(subpacket);
    }
    (read, subpackets)
}

fn read_children_bits(it: &mut BitStream) -> (u32, Vec<Packet>) {
    let mut read = 0;
    let mut subpacket_bit_length = to_num(&read_n(it, 15));
    read += 15;

    let mut subpackets: Vec<Packet> = Vec::new();
    while subpacket_bit_length > 0 {
        let (n, subpacket) = parse_packet(it);
        read += n;
        subpacket_bit_length -= n as u64;
        subpackets.push(subpacket);
    }
    (read, subpackets)
}

fn parse_packet(it: &mut BitStream) -> (u32, Packet) {
    let mut read = 0;

    let version: u8 = to_num(&read_n(it, 3)).try_into().unwrap();
    read += 3;
    let r#type: u8 = to_num(&read_n(it, 3)).try_into().unwrap();
    read += 3;

    let mut p = Packet::empty(version, r#type);
    match r#type {
        // literal
        4 => {
            let (n, literal) = read_literal(it);
            read += n;
            p.value = Some(literal);
        }
        // operator
        _ => {
            let length_type_id = read_n(it, 1)[0];
            read += 1;

            let (n, subpackets) = match length_type_id {
                0 => read_children_bits(it),
                1 => read_children_n(it),
                _ => panic!("unexpected length type id"),
            };
            read += n;
            p.children = subpackets;
        }
    }
    (read, p)
}

fn solve (v: &[u8], sum_headers: bool) -> u64 {
    let mut it = v.iter();
    let (_, p) = parse_packet(&mut it);

    if sum_headers {
        let mut q: VecDeque<Packet> = VecDeque::new();
        q.push_back(p);

        let mut sum = 0;
        while !q.is_empty() {
            let p = q.pop_front().unwrap();
            sum += p.version as u64;
            q.extend(p.children);
        }
        sum
    } else {
        p.value()
    }
}

fn main() {
    let args = Cli::from_args();
    let input = read_input();
    let answer = solve(&input, args.part == 1);
    println!("{}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_packet(version: u8, r#type: u8, value: Option<u64>, children: &[Packet]) -> Packet {
        let mut p = Packet::empty(version, r#type);
        p.value = value;
        p.children = children.to_vec();
        p
    }

    #[test]
    fn literal() {
        let input = to_bits("D2FE28");
        let mut it = input.iter();
        let (_, p) = parse_packet(&mut it);
        assert_eq!(p, new_packet(6, 4, Some(2021), &[]));
    }

    #[test]
    fn operator_with_bit_length() {
        let input = to_bits("38006F45291200");
        let mut it = input.iter();
        let (_, p) = parse_packet(&mut it);
        assert_eq!(p,
            new_packet(1, 6, None, &[
                new_packet(6, 4, Some(10), &[]),
                new_packet(2, 4, Some(20), &[]),
            ])
        );
    }

    #[test]
    fn operator_nested_1() {
        let input = to_bits("8A004A801A8002F478");
        let mut it = input.iter();
        let (_, p) = parse_packet(&mut it);
        assert_eq!(p,
            new_packet(4, 2, None, &[
                new_packet(1, 2, None, &[
                    new_packet(5, 2, None, &[
                        new_packet(6, 4, Some(15), &[])
                    ])
                ])
            ])
        );
    }

    #[test]
    fn operator_nested_2() {
        let input = to_bits("A0016C880162017C3686B18A3D4780");
        let mut it = input.iter();
        let (_, p) = parse_packet(&mut it);
        assert_eq!(p,
            new_packet(5, 0, None, &[
                new_packet(1, 0, None, &[
                    new_packet(3, 0, None, &[
                        new_packet(7, 4, Some(6), &[]),
                        new_packet(6, 4, Some(6), &[]),
                        new_packet(5, 4, Some(12), &[]),
                        new_packet(2, 4, Some(15), &[]),
                        new_packet(2, 4, Some(15), &[]),
                    ])
                ])
            ])
        );
    }

    #[test]
    fn operator_subpackets_1() {
        let input = to_bits("620080001611562C8802118E34");
        let mut it = input.iter();
        let (_, p) = parse_packet(&mut it);
        assert_eq!(p,
            new_packet(3, 0, None, &[
                new_packet(0, 0, None, &[
                    new_packet(0, 4, Some(10), &[]),
                    new_packet(5, 4, Some(11), &[]),
                ]),
                new_packet(1, 0, None, &[
                    new_packet(0, 4, Some(12), &[]),
                    new_packet(3, 4, Some(13), &[]),
                ]),
            ])
        );
    }
}
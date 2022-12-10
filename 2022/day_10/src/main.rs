use nom::branch::alt;

use nom::combinator::map;
use nom::number::complete::float;
use nom::sequence::preceded;
use nom::{bytes::complete::tag, IResult};

#[derive(Debug)]
enum TermLine {
    Noop,
    Addx(i32),
}

fn num_parser(l: &str) -> IResult<&str, i32> {
    map(float, |x| x as i32)(l)
}

fn line_parser(l: &str) -> IResult<&str, TermLine> {
    alt((
        map(tag("noop"), |_| TermLine::Noop),
        map(preceded(tag("addx "), num_parser), TermLine::Addx),
    ))(l)
}

fn data() -> impl Iterator<Item = TermLine> {
    include_str!("../i.txt")
        .lines()
        .map(|b| line_parser(b).unwrap().1)
}

fn add_r(i: i32, register: i32) -> i32 {
    if (i - 20) % 40 == 0 {
        return i * register;
    }
    0
}

fn part1() {
    let mut cycle = 0;
    let mut register = 1;
    let mut r = 0;

    for v in data() {
        cycle += 1;
        match v {
            TermLine::Noop => r += add_r(cycle, register),
            TermLine::Addx(x) => {
                r += add_r(cycle, register);
                cycle += 1;
                r += add_r(cycle, register);
                register += x;
            }
        }
    }
    println!("{}", r)
}

fn draw(cycle: i32, lines: &mut [[char; 40]; 6], register: i32) {
    let line_id = (cycle - 1) / 40;
    let v_pos = (cycle - 1) % 40;

    if v_pos.abs_diff(register) < 2 {
        lines[line_id as usize][v_pos as usize] = '#';
    }
}

fn print(lines: &[[char; 40]; 6]) {
    for line in lines.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn part2() {
    let mut cycle = 0;
    let mut register = 1;
    let mut lines = [['.'; 40]; 6];

    for v in data() {
        cycle += 1;
        match v {
            TermLine::Noop => draw(cycle, &mut lines, register),
            TermLine::Addx(x) => {
                draw(cycle, &mut lines, register);
                cycle += 1;
                draw(cycle, &mut lines, register);
                register += x;
            }
        }
    }
    print(&lines);
}

fn main() {
    part1();
    part2();
}

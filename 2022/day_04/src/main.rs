use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_opt,
    sequence::separated_pair,
    IResult,
};
use atoi::atoi;

fn num_parser(l: &[u8]) -> IResult<&[u8], usize> {
    map_opt(digit1, atoi)(l)
}

fn coord_parser(l: &[u8]) -> IResult<&[u8], (usize, usize)> {
    separated_pair(num_parser, char('-'), num_parser)(l)
}

fn line_parser(l: &[u8]) -> IResult<&[u8], ((usize, usize), (usize, usize))> {
    separated_pair(coord_parser, tag(","), coord_parser)(l)
}

fn part1() {
    let a = include_bytes!("../i.txt")
        .split(|x| *x == b'\n')
        .map(|entry| line_parser(entry).unwrap().1)
        .filter(|((x, y), (xx, yy))| (x >= xx && y <= yy) || (x <= xx && y >= yy))
        .count();
    println!("{}", a);
}

fn part2() {
    let a = include_bytes!("../i.txt")
        .split(|x| *x == b'\n')
        .map(|entry| line_parser(entry).unwrap().1)
        .filter(|((x, y), (xx, yy))| {
            (x <= xx && xx <= y) || (x <= yy && yy <= y) || (x >= xx && y <= yy) || (x <= xx && y >= yy)
        })
        .count();
    println!("{}", a);
}

fn main() {
    part1();
    part2();
}

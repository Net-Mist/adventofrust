use nom::combinator::map;
use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, sequence::tuple, IResult};
use std::collections::VecDeque;

fn num_parser(l: &str) -> IResult<&str, usize> {
    map_res(digit1, |c: &str| c.parse::<usize>())(l)
}

fn line_parser(l: &str) -> IResult<&str, (usize, usize, usize)> {
    map(
        tuple((
            tag("move "),
            num_parser,
            tag(" from "),
            num_parser,
            tag(" to "),
            num_parser,
        )),
        |(_, a, _, b, _, c)| (a, b, c),
    )(l)
}

fn data() -> (Vec<VecDeque<char>>, impl Iterator<Item = (usize, usize, usize)>) {
    let mut stacks = vec![VecDeque::new(); 9];
    let (p1, p2) = include_str!("../i.txt").split_once("\n\n").unwrap();
    p1.split('\n').for_each(|line| {
        let mut l_iter = line.chars();
        l_iter.next();
        l_iter.step_by(4).enumerate().for_each(|(i, e)| {
            if e != ' ' {
                stacks[i].push_back(e)
            }
        })
    });

    let actions = p2.lines().map(|entry| line_parser(entry).unwrap().1);

    (stacks, actions)
}

fn part1() {
    let (mut stacks, actions) = data();

    actions.for_each(|(q, or, des)| {
        (0..q).for_each(|_| {
            let e = stacks[or - 1].pop_front().unwrap();
            stacks[des - 1].push_front(e);
        });
    });

    stacks
        .into_iter()
        .for_each(|mut q| print!("{}", q.pop_front().unwrap()));
    println!()
}

fn part2() {
    let (mut stacks, actions) = data();

    actions.for_each(|(q, or, des)| {
        let mut int = VecDeque::new();
        (0..q).for_each(|_| {
            int.push_front(stacks[or - 1].pop_front());
        });
        (0..q).for_each(|_| {
            stacks[des - 1].push_front(int.pop_front().unwrap().unwrap());
        });
    });

    stacks
        .into_iter()
        .for_each(|mut q| print!("{}", q.pop_front().unwrap()));
    println!()
}

fn main() {
    part1();
    part2();
}

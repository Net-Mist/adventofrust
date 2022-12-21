use nom::branch::alt;
use nom::bytes::complete::take;
use nom::bytes::complete::{take_until1};
use nom::combinator::map;

use nom::number::complete::float;

use nom::{bytes::complete::tag, sequence::tuple, IResult};
use std::collections::HashMap;



#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum((&'static str, &'static str)),
    Sub((&'static str, &'static str)),
    Mult((&'static str, &'static str)),
    Div((&'static str, &'static str)),
    Value(i128),
}

fn num_parser(l: &str) -> IResult<&str, u32> {
    map(float, |x| x as u32)(l)
}

fn parser(b: &'static str) -> IResult<&str, (&str, Operation)> {
    alt((
        map(
            tuple((
                take_until1(":"), //6
                tag(": "),
                num_parser,
            )),
            |r| (r.0, Operation::Value(r.2 as i128)),
        ),
        map(
            tuple((
                take_until1(":"),
                tag(": "),
                take_until1(" "),
                tag(" "),
                take_until1(" "), //4
                tag(" "),
                take(4usize),
            )),
            |r| {
                let operation = match r.4 {
                    "+" => Operation::Sum((r.2, r.6)),
                    "-" => Operation::Sub((r.2, r.6)),
                    "*" => Operation::Mult((r.2, r.6)),
                    "/" => Operation::Div((r.2, r.6)),
                    _ => unreachable!(),
                };
                (r.0, operation)
            },
        ),
    ))(b)
}

fn data() -> HashMap<&'static str, Operation> {
    include_str!("../i.txt")
        .lines()
        .map(|b| parser(b).unwrap().1)
        .collect()
}


fn part1() {
    let mut monkeys = data();
    println!("{:?}", monkeys);

    let mut values = HashMap::new();

    while !values.contains_key(&"root") {
        let mut val_to_remove = Vec::new();
        for (&k, v) in monkeys.iter() {
            if let Operation::Value(value) = v {
                values.insert(k, *value);
                val_to_remove.push(k);
            }
            match v {
                Operation::Sum((a, b)) => {
                    if values.contains_key(a) && values.contains_key(b) {
                        let val = *values.get(a).unwrap() + *values.get(b).unwrap();
                        values.insert(k, val);
                        val_to_remove.push(k);
                    }
                }
                Operation::Sub((a, b)) => {
                    if values.contains_key(a) && values.contains_key(b) {
                        let val = *values.get(a).unwrap() - *values.get(b).unwrap();
                        values.insert(k, val);
                        val_to_remove.push(k);
                    }
                }
                Operation::Mult((a, b)) => {
                    if values.contains_key(a) && values.contains_key(b) {
                        let val = *values.get(a).unwrap() * *values.get(b).unwrap();
                        values.insert(k, val);
                        val_to_remove.push(k);
                    }
                }
                Operation::Div((a, b)) => {
                    if values.contains_key(a) && values.contains_key(b) {
                        let val = *values.get(a).unwrap() / *values.get(b).unwrap();
                        values.insert(k, val);
                        val_to_remove.push(k);
                    }
                }
                _ => {}
            }
        }
        for v in val_to_remove.into_iter() {
            monkeys.remove(v);
        }
    }
    println!("{:?}", values.get(&"root"));
}

// for part 2 implement full tree
#[derive(Debug, Clone, Copy)]
struct Op {
    op: &'static str,
    node_i_l: &'static str,
    node_i_r: &'static str,
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Value(i128),
    Unknown,
    Op(Op)
}

fn do_op(op: &Operation, v1: i128, v2: i128) -> i128{
    match op {
        Operation::Sum(_) => {v1+v2},
        Operation::Sub(_) => {v1-v2},
        Operation::Mult(_) => {v1*v2},
        Operation::Div(_) => {v1/v2},
        _ => unreachable!()
    }
}

fn part2() {
    let mut monkeys = data();
    println!("{:?}", monkeys);
    let mut values = HashMap::new();

    // fill values
    for (n, m) in monkeys.into_iter() {

    }
}


fn main() {
    // part1();
    part2();
}
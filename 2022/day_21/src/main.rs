use nom::branch::alt;
use nom::bytes::complete::take;
use nom::bytes::complete::{take_until1};
use nom::combinator::map;

use nom::number::complete::float;

use nom::{bytes::complete::tag, sequence::tuple, IResult};
use std::collections::HashMap;

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



fn num_parser(l: &str) -> IResult<&str, u32> {
    map(float, |x| x as u32)(l)
}

fn parser(b: &'static str) -> IResult<&str, (&str, Node)> {
    alt((
        map(
            tuple((
                take_until1(":"), //6
                tag(": "),
                num_parser,
            )),
            |r| (r.0, Node::Value(r.2 as i128)),
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
                (r.0, Node::Op( Op {
                    op: r.4,
                    node_i_l: r.2,
                    node_i_r: r.6
                }))
            },
        ),
    ))(b)
}

fn data() -> HashMap<&'static str, Node> {
    include_str!("../i.txt")
        .lines()
        .map(|b| parser(b).unwrap().1)
        .collect()
}

fn do_op(op: &str, a:i128, b:i128) -> i128{
    match op {
        "+" => {a + b},
        "-" => {a - b},
        "*" => {a * b},
        "/" => {a / b},
        _ => unreachable!()
    }
}

fn do_reverse_op_l(op: &str, a:i128, r:i128) -> i128{
    match op {
        "+" => {r - a},
        "-" => {a - r},
        "*" => {r / a},
        "/" => {a / r},
        _ => unreachable!()
    }
}

fn do_reverse_op_r(op: &str, a:i128, r:i128) -> i128{
    match op {
        "+" => {r - a},
        "-" => {a + r},
        "*" => {r / a},
        "/" => {a * r},
        _ => unreachable!()
    }
}

fn part1() {
    let mut monkeys = data();
    loop {
        let mut value_to_insert = Vec::new();
        let mut value_to_remove = Vec::new();

        for &k in monkeys.keys() {
            let node =  monkeys.get(k).unwrap();
            match node {
                Node::Value(_) | Node::Unknown => continue,
                Node::Op(op) => {
                    match (monkeys.get(op.node_i_l).unwrap(), monkeys.get(op.node_i_r).unwrap()) {
                        (Node::Value(a), Node::Value(b)) => {
                            let v = do_op(op.op, *a, *b);
                            value_to_insert.push((k, Node::Value(v)));
                            value_to_remove.push(op.node_i_l);
                            value_to_remove.push(op.node_i_r);
                        }
                        _ => {}
                    }
                }
            }
        }

        if value_to_insert.is_empty() {
            break;
        }
        for (k, v) in value_to_insert {
            monkeys.insert(k, v);
        }
        for k in value_to_remove {
            monkeys.remove(k);
        }
    }
    println!("{:?}", monkeys.get("root").unwrap());
}

fn part2() {
    let mut monkeys = data();

    // correct mistakes
    let root_op = monkeys.get(&"root").unwrap();
    let mut root_left = "";
    let mut root_right = "";
    let new_root_op = match root_op {
        Node::Op(op) => {
            root_left = op.node_i_l;
            root_right = op.node_i_r;
            Op {
                op: &"=",
                node_i_l: op.node_i_l,
                node_i_r: op.node_i_r
            }
        }
        _ => unreachable!()
    };
    monkeys.insert(&"root", Node::Op(new_root_op));
    monkeys.insert(&"humn", Node::Unknown);

    // simplify tree
    loop {
        let mut value_to_insert = Vec::new();
        let mut value_to_remove = Vec::new();

        for &k in monkeys.keys() {
            let node =  monkeys.get(k).unwrap();
            match node {
                Node::Value(_) | Node::Unknown=> continue,
                Node::Op(op) => {
                    match (monkeys.get(op.node_i_l).unwrap(), monkeys.get(op.node_i_r).unwrap()) {
                        (Node::Value(a), Node::Value(b)) => {
                            let v = do_op(op.op, *a, *b);
                            value_to_insert.push((k, Node::Value(v)));
                            value_to_remove.push(op.node_i_l);
                            value_to_remove.push(op.node_i_r);
                        }
                        _ => {}
                    }
                }
            }
        }

        if value_to_insert.is_empty() {
            break;
        }

        for (k, v) in value_to_insert {
            monkeys.insert(k, v);
        }
        for k in value_to_remove {
            monkeys.remove(k);
        }
    }

    // solve
    let mut node_to_solve;
    let mut value;
    match (monkeys.get(root_left).unwrap(), monkeys.get(root_right).unwrap()) {
        (Node::Value(v), _ ) => {value = *v; node_to_solve=root_right},
        (_, Node::Value(v))  => {value = *v; node_to_solve=root_left},
        _ => unreachable!()
    }

    while node_to_solve != "humn" {
        let node = monkeys.get(node_to_solve).unwrap();
        match node {
            Node::Op(op) => {
                match (monkeys.get(op.node_i_l).unwrap(), monkeys.get(op.node_i_r).unwrap()) {
                    (Node::Value(a), _) => {
                        value = do_reverse_op_l(op.op, *a, value);
                        node_to_solve = op.node_i_r
                    }
                    (_, Node::Value(a)) => {
                        value = do_reverse_op_r(op.op, *a, value);
                        node_to_solve = op.node_i_l
                    }
                    _ => {
                        panic!()
                    }
                    
                }
            },
            _ => unreachable!()
        }

    }
    println!("{:?}", value);


    
}


fn main() {
    part1();
    part2();
}
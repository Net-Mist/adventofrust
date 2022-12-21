#![feature(test)]

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until1},
    combinator::map,
    number::complete::float,
    sequence::tuple,
    IResult,
};

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
    Op(Op),
}

fn num_parser(l: &str) -> IResult<&str, u32> {
    map(float, |x| x as u32)(l)
}

fn parser(b: &'static str) -> IResult<&str, (&str, Node)> {
    alt((
        map(tuple((take_until1(":"), tag(": "), num_parser)), |r| (r.0, Node::Value(r.2 as i128))),
        map(
            tuple((take_until1(":"), tag(": "), take_until1(" "), tag(" "), take_until1(" "), tag(" "), take(4usize))),
            |r| (r.0, Node::Op(Op { op: r.4, node_i_l: r.2, node_i_r: r.6 })),
        ),
    ))(b)
}

fn data() -> HashMap<&'static str, Node> {
    include_str!("../i.txt").lines().map(|b| parser(b).unwrap().1).collect()
}

fn do_op(op: &str, a: i128, b: i128) -> i128 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => unreachable!(),
    }
}

fn do_reverse_op_l(op: &str, a: i128, r: i128) -> i128 {
    match op {
        "+" => r - a,
        "-" => a - r,
        "*" => r / a,
        "/" => a / r,
        _ => unreachable!(),
    }
}

fn do_reverse_op_r(op: &str, a: i128, r: i128) -> i128 {
    match op {
        "+" => r - a,
        "-" => a + r,
        "*" => r / a,
        "/" => a * r,
        _ => unreachable!(),
    }
}

fn compute_value(node: &Node, all_nodes: &HashMap<&str, Node>) -> Result<i128, &'static str> {
    match node {
        Node::Unknown => Err("unknown variable"),
        Node::Value(v) => Ok(*v),
        Node::Op(op) => {
            let Ok(l) = compute_value(all_nodes.get(&op.node_i_l).unwrap(), all_nodes) else {return Err("unknown variable")};
            let Ok(r) = compute_value(all_nodes.get(&op.node_i_r).unwrap(), all_nodes) else {return Err("unknown variable")};
            Ok(do_op(op.op, l, r))
        }
    }
}

fn propagate_value(node: &Op, all_nodes: &HashMap<&str, Node>, value: i128) -> i128 {
    let l = all_nodes.get(&node.node_i_l).unwrap();
    let r = all_nodes.get(&node.node_i_r).unwrap();
    match (compute_value(l, all_nodes), compute_value(r, all_nodes)) {
        (Ok(v), Err(_)) => {
            let v = do_reverse_op_l(node.op, v, value);
            if let Node::Op(op) = r {
                propagate_value(op, all_nodes, v)
            } else {
                v
            }
        }
        (Err(_), Ok(v)) => {
            let v = do_reverse_op_r(node.op, v, value);
            if let Node::Op(op) = l {
                propagate_value(op, all_nodes, v)
            } else {
                v
            }
        }
        _ => unreachable!(),
    }
}

fn part1() {
    let monkeys = data();
    println!("{:?}", compute_value(monkeys.get("root").unwrap(), &monkeys));
}

fn part2() {
    let mut monkeys = data();

    // correct mistakes
    let (root_left, root_right) =
        if let Node::Op(Op { op: _, node_i_l: root_left, node_i_r: root_right }) = monkeys.get(&"root").unwrap() {
            (*root_left, *root_right)
        } else {
            panic!()
        };

    monkeys.insert("root", Node::Op(Op { op: "=", node_i_l: root_left, node_i_r: root_right }));
    monkeys.insert("humn", Node::Unknown);

    if let Ok(v) = compute_value(monkeys.get(root_left).unwrap(), &monkeys) {
        let Node::Op(op) = monkeys.get(root_right).unwrap() else {panic!()};
        println!("{}", propagate_value(op, &monkeys, v));
    } else if let Ok(v) = compute_value(monkeys.get(root_right).unwrap(), &monkeys) {
        let Node::Op(op) = monkeys.get(root_left).unwrap() else {panic!()};
        println!("{}", propagate_value(op, &monkeys, v));
    }
}

fn main() {
    part1();
    part2();
}

fn part1_1() {
    let mut monkeys = data();
    loop {
        let mut value_to_insert = Vec::new();
        let mut value_to_remove = Vec::new();

        for &k in monkeys.keys() {
            if let Node::Op(op) = monkeys.get(k).unwrap() {
                if let (Node::Value(a), Node::Value(b)) =
                    (monkeys.get(op.node_i_l).unwrap(), monkeys.get(op.node_i_r).unwrap())
                {
                    let v = do_op(op.op, *a, *b);
                    value_to_insert.push((k, Node::Value(v)));
                    value_to_remove.push(op.node_i_l);
                    value_to_remove.push(op.node_i_r);
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
}

fn part2_1() {
    let mut monkeys = data();

    // correct mistakes
    let (&root_left, &root_right) =
        if let Node::Op(Op { op: _, node_i_l: root_left, node_i_r: root_right }) = monkeys.get(&"root").unwrap() {
            (root_left, root_right)
        } else {
            panic!()
        };
    monkeys.insert("root", Node::Op(Op { op: "=", node_i_l: root_left, node_i_r: root_right }));
    monkeys.insert("humn", Node::Unknown);

    // simplify tree
    loop {
        let mut value_to_insert = Vec::new();
        let mut value_to_remove = Vec::new();

        for &k in monkeys.keys() {
            let node = monkeys.get(k).unwrap();

            if let Node::Op(op) = node {
                if let (Node::Value(a), Node::Value(b)) =
                    (monkeys.get(op.node_i_l).unwrap(), monkeys.get(op.node_i_r).unwrap())
                {
                    let v = do_op(op.op, *a, *b);
                    value_to_insert.push((k, Node::Value(v)));
                    value_to_remove.push(op.node_i_l);
                    value_to_remove.push(op.node_i_r);
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
        (Node::Value(v), _) => {
            value = *v;
            node_to_solve = root_right
        }
        (_, Node::Value(v)) => {
            value = *v;
            node_to_solve = root_left
        }
        _ => unreachable!(),
    }

    while node_to_solve != "humn" {
        let node = monkeys.get(node_to_solve).unwrap();
        match node {
            Node::Op(op) => match (monkeys.get(op.node_i_l).unwrap(), monkeys.get(op.node_i_r).unwrap()) {
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
            },
            _ => unreachable!(),
        }
    }
    println!("{value:?}");
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_part1_1(b: &mut Bencher) {
        b.iter(part1_1)
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1)
    }

    #[bench]
    fn bench_part2_1(b: &mut Bencher) {
        b.iter(part2_1)
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2)
    }
}

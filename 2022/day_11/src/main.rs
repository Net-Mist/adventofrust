use nom::bytes::complete::{take_until, take_until1};
use nom::combinator::map;
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::preceded;
use nom::{bytes::complete::tag, sequence::tuple, IResult};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Square,
    Sum(u32),
    Mult(u32),
}

#[derive(Debug)]
struct Monkey1 {
    items: VecDeque<u32>,
    operation: Operation,
    test: u32,
    monkey_id_true: usize,
    monkey_id_false: usize,
    inspected: usize,
}

fn num_parser(l: &str) -> IResult<&str, u32> {
    map(float, |x| x as u32)(l)
}

fn parser(b: &str) -> IResult<&str, Monkey1> {
    map(
        tuple((
            tag("Monkey "),
            num_parser,
            tag(":\n  Starting items: "),
            num_parser,                             //3
            many0(preceded(tag(", "), num_parser)), //4
            tag("\n  Operation: new = old "),
            take_until1(" "), //6
            tag(" "),
            take_until("\n"), //8
            tag("\n  Test: divisible by "),
            num_parser,
            tag("\n    If true: throw to monkey "),
            num_parser,
            tag("\n    If false: throw to monkey "),
            num_parser,
        )),
        |r| {
            let mut items = VecDeque::<u32>::from(r.4);
            items.push_front(r.3);
            let operation = match (r.6, r.8) {
                ("*", "old") => Operation::Square,
                ("*", n) => Operation::Mult(n.parse::<u32>().unwrap()),
                ("+", n) => Operation::Sum(n.parse::<u32>().unwrap()),
                (a, b) => {
                    println!("'{a}', '{b}'");
                    panic!()
                }
            };
            Monkey1 {
                items,
                operation,
                test: r.10,
                monkey_id_true: r.12 as usize,
                monkey_id_false: r.14 as usize,
                inspected: 0,
            }
        },
    )(b)
}

fn data() -> Vec<Monkey1> {
    include_str!("../i.txt")
        .split("\n\n")
        .map(|b| parser(b).unwrap().1)
        .collect::<Vec<Monkey1>>()
}

fn do_operation(op: &Operation, n: u32) -> u32 {
    match op {
        Operation::Square => n * n,
        Operation::Sum(m) => n + m,
        Operation::Mult(m) => n * m,
    }
}

fn part1() {
    let mut monkeys = data();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let item = monkeys[i].items.pop_front().unwrap();
                monkeys[i].inspected += 1;
                let new = do_operation(&monkeys[i].operation, item) / 3;
                if new % monkeys[i].test == 0 {
                    let nid = monkeys[i].monkey_id_true;
                    monkeys[nid].items.push_back(new)
                } else {
                    let nid = monkeys[i].monkey_id_false;
                    monkeys[nid].items.push_back(new)
                }
            }
        }
    }
    let mut a = monkeys
        .into_iter()
        .map(|m| m.inspected)
        .collect::<Vec<usize>>();
    a.sort();
    println!("{:?}", a.into_iter().rev().take(2).reduce(|a, b| a * b));
}

#[derive(Debug)]
struct Monkey2 {
    items: VecDeque<Item>,
    operation: Operation,
    test: u32,
    monkey_id_true: usize,
    monkey_id_false: usize,
    inspected: usize,
}

#[derive(Debug)]
struct Item {
    r: HashMap<u32, u32>,
}

impl Item {
    fn from(n: u32) -> Self {
        let mut s = HashMap::<u32, u32>::new();
        for d in vec![2, 3, 5, 7, 11, 13, 17, 19, 23].into_iter() {
            s.insert(d, n % d);
        }
        Item { r: s }
    }

    fn add(self, n: u32) -> Self {
        let h = self.r.into_iter().map(|(k, v)| (k, (v + n) % k)).collect();
        Item { r: h }
    }

    fn mult(self, n: u32) -> Self {
        let h = self.r.into_iter().map(|(k, v)| (k, (v * n) % k)).collect();
        Item { r: h }
    }

    fn square(self) -> Self {
        let h = self.r.into_iter().map(|(k, v)| (k, (v * v) % k)).collect();
        Item { r: h }
    }

    fn test(&self, n: u32) -> bool {
        *self.r.get(&n).unwrap() == 0
    }
}

fn do_operation2(op: &Operation, n: Item) -> Item {
    match op {
        Operation::Square => n.square(),
        Operation::Sum(m) => n.add(*m),
        Operation::Mult(m) => n.mult(*m),
    }
}

fn part2() {
    let monkeys = data();
    let mut monkeys = monkeys
        .into_iter()
        .map(|m| Monkey2 {
            items: m.items.into_iter().map(Item::from).collect(),
            operation: m.operation,
            test: m.test,
            monkey_id_true: m.monkey_id_true,
            monkey_id_false: m.monkey_id_false,
            inspected: m.inspected,
        })
        .collect::<Vec<Monkey2>>();

    for _j in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let item = monkeys[i].items.pop_front().unwrap();
                monkeys[i].inspected += 1;

                let new = do_operation2(&monkeys[i].operation, item);

                if new.test(monkeys[i].test) {
                    let nid = monkeys[i].monkey_id_true;

                    monkeys[nid].items.push_back(new)
                } else {
                    let nid = monkeys[i].monkey_id_false;

                    monkeys[nid].items.push_back(new)
                }
            }
        }
    }

    let mut a = monkeys
        .into_iter()
        .map(|m| m.inspected)
        .collect::<Vec<usize>>();
    a.sort();
    println!("{:?}", a.into_iter().rev().take(2).reduce(|a, b| a * b));
}

fn main() {
    part1();
    part2();
}

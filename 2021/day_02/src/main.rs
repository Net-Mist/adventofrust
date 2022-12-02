#[derive(Debug)]
struct Data {
    verb: Verb,
    n: i32,
}

#[derive(Debug)]
enum Verb {
    Forward,
    Down,
    Up,
}

fn data() -> impl Iterator<Item = Data> {
    include_str!("../i.txt").lines().map(|l| {
        let (v, n) = l.split_once(' ').unwrap();
        let verb = match v {
            "forward" => Verb::Forward,
            "down" => Verb::Down,
            "up" => Verb::Up,
            _ => unreachable!(),
        };
        Data { verb, n: n.parse().unwrap() }
    })
}

fn part1() {
    let (f, d) = data().fold((0, 0), |(f, d), x| match x {
        Data { verb: Verb::Forward, n } => (f + n, d),
        Data { verb: Verb::Up, n } => (f, d - n),
        Data { verb: Verb::Down, n } => (f, d + n),
    });
    println!("{:?}", f * d);
}

fn part2() {
    let (f, d, _a) = data().fold((0, 0, 0), |(f, d, a), x| match x {
        Data { verb: Verb::Forward, n } => (f + n, d + a * n, a),
        Data { verb: Verb::Up, n } => (f, d, a - n),
        Data { verb: Verb::Down, n } => (f, d, a + n),
    });
    println!("{:?}", f * d);
}

fn main() {
    part1();
    part2();
}

use std::collections::{HashSet, VecDeque};

type Point = (i32, i32, i32);
const NEIGHBORED: [Point; 6] = [
    (0, 0, 1),
    (0, 0, -1),
    (0, 1, 0),
    (0, -1, 0),
    (1, 0, 0),
    (-1, 0, 0),
];

fn data() -> HashSet<Point> {
    include_str!("../i.txt")
        .lines()
        .map(|l| {
            let pos: Vec<_> = l.split(',').map(|n| n.parse().unwrap()).collect();
            (
                *pos.first().unwrap(),
                *pos.get(1).unwrap(),
                *pos.get(2).unwrap(),
            )
        })
        .collect()
}

fn neighbors(p: &Point) -> impl Iterator<Item = Point> + '_ {
    NEIGHBORED.iter().map(|v| (v.0 + p.0, v.1 + p.1, v.2 + p.2))
}

fn part1() {
    let d = data();

    let mut n_faces = 0;
    for p in d.iter() {
        n_faces += neighbors(p).filter(|p| !d.contains(p)).count();
    }
    println!("{:?}", n_faces);
}

fn part2() {
    let d = data();
    let (max_x, max_y, max_z) = d
        .iter()
        .cloned()
        .reduce(|accum, item| {
            (
                accum.0.max(item.0),
                accum.1.max(item.1),
                accum.2.max(item.2),
            )
        })
        .unwrap();
    println!("{max_x}, {max_y}, {max_z}");
    // flood water from 0, 0, 0
    let mut hash_water = HashSet::new();
    hash_water.insert((0, 0, 0));
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, 0, 0));
    let mut n_faces = 0;
    while !to_visit.is_empty() {
        let p = to_visit.pop_front().unwrap();
        for i in neighbors(&p).filter(|v| {
            v.0 >= 0 && v.0 <= max_x && v.1 >= 0 && v.1 <= max_y && v.2 >= 0 && v.2 <= max_z
        }) {
            if d.contains(&i) {
                n_faces += 1;
            } else if !hash_water.contains(&i) {
                to_visit.push_back(i);
                hash_water.insert(i);
            }
        }
    }
    println!("{}", n_faces)
}

fn main() {
    part1();
    part2();
}

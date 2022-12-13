use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use petgraph::graph::{NodeIndex, DiGraph};
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};

fn data() -> (
    Vec<((usize, usize), (usize, usize))>,
    (usize, usize),
    Vec<(usize, usize)>,
    (usize, usize),
    Vec<Vec<char>>,
) {
    let mut connections = Vec::<((usize, usize), (usize, usize))>::new();
    let file = include_str!("../i.txt");
    let mut map = file
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let h = map.len();
    let w = map[0].len();

    let mut s = (0, 0);
    let mut s_part_2 = Vec::new();
    let mut e = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match map[i][j] {
                'S' => {
                    map[i][j] = 'a';
                    s = (i, j);
                }
                'E' => {
                    map[i][j] = 'z';
                    e = (i, j);
                }
                'a' => {
                    s_part_2.push((i, j));
                }
                _ => {}
            }
        }
    }

    for i in 1..h {
        for j in 0..w {
            if (map[i - 1][j] as i32) - (map[i][j] as i32) < 2 {
                connections.push(((i, j), (i - 1, j)));
            }
            if (map[i][j] as i32) - (map[i - 1][j] as i32) < 2 {
                connections.push(((i - 1, j), (i, j)));
            }
        }
    }

    for i in 0..h {
        for j in 1..w {
            if (map[i][j - 1] as i32) - (map[i][j] as i32) < 2 {
                connections.push(((i, j), (i, j - 1)));
            }
            if (map[i][j] as i32) - (map[i][j - 1] as i32) < 2 {
                connections.push(((i, j - 1), (i, j)));
            }
        }
    }

    (connections, s, s_part_2, e, map)
}


fn explore(
    mut cost: HashMap<(usize, usize), usize>,
    mut to_process: BinaryHeap<Reverse<(i32, (usize, usize))>>,
    _map: &Vec<Vec<char>>,
    connections: &Vec<((usize, usize), (usize, usize))>,
    e: &(usize, usize),
) {
    while !cost.contains_key(&e) {
        let (score, pos) = to_process.pop().unwrap().0;
        for conn in connections.iter() {
            if conn.0 == pos {
                if !cost.contains_key(&conn.1) {
                    cost.insert(conn.1, cost.get(&pos).unwrap() + 1);
                    to_process.push(Reverse((score + 1, conn.1)))
                }
            }
        }
    }
    println!("{}", cost.get(&e).unwrap())
}

fn part12() {
    let (connections, s, s_part2, e, map) = data();

    // part 1
    let mut cost = HashMap::<(usize, usize), usize>::new(); // position, score
    let mut to_process = BinaryHeap::new(); // score, position

    to_process.push(Reverse((0, s)));
    cost.insert(s, 0);
    explore(cost, to_process, &map, &connections, &e);

    // part 2
    let mut cost = HashMap::<(usize, usize), usize>::new(); // position, score
    let mut to_process = BinaryHeap::new(); // score, position

    for s in s_part2.into_iter() {
        to_process.push(Reverse((0, s)));
        cost.insert(s, 0);
    }
    explore(cost, to_process, &map, &connections, &e);
}

// fn part1_petgraph() {
//     let (connections, s, s_part2, e, map) = data();

//     let g = DiGraph::<(usize, usize), ()>::from_edges(connections);

//     // Into<NodeIndex<Ix> for (usize, usize)

// }

fn main() {
    part12();
    // part1_petgraph();
}

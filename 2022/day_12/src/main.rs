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

#[derive(Default)]
struct Position {
    x: usize,
    y: usize
}

impl From<Position> for NodeIndex {
    fn from(ix: Position) -> Self {
        NodeIndex::new(ix.x + ix.y * 200)
    }
}

impl From<(usize, usize)> for Position {
    fn from(ix: (usize, usize)) -> Self {
        Position {x: ix.0, y: ix.1 }
    }
}

fn part1_petgraph() {
    let (connections, s, s_part2, e, map) = data();

    let g = DiGraph::<Position, ()>::from_edges(connections.into_iter().map(|(a, b)| {
        (Position::from(b), Position::from(a))
    }));

    let a = dijkstra(&g, Position::from(e).into(), Some(Position::from(s).into()), |_| 1);
    println!("{:?}", a.get(&Position::from(s).into()));

    let a = s_part2.into_iter().map(|s| Position::from(s)).filter_map(|p| {
        let i = p.into();
        if a.contains_key(&i) {
            Some(a.get(&i).unwrap())
        } else {
            None
        }
    }).min().unwrap();
    println!("{:?}", a);
}


/// https://github.com/timvisee/advent-of-code-2022/blob/master/day12a/src/main.rs
fn part1_pathfinding() {
    let data = include_bytes!("../i.txt");
    let w = data.iter().position(|b| *b == b'\n').unwrap();
    let data = data.into_iter().filter(|b| **b != b'\n').collect::<Vec<_>>();
    let mut map = data.iter().map(|b| b.to_ascii_lowercase() - b'a').collect::<Vec<_>>();
    let h = map.len() / w;

    let start = data.iter().position(|p| **p == b'S').unwrap();
    let end = data.iter().position(|p| **p == b'E').unwrap();

    map[start] = 0;
    map[end] = 25;

    let possible_neighbours = [(1,0), (0, 1), (usize::MAX, 0), (0, usize::MAX)];

    let a= pathfinding::directed::bfs::bfs(&(start/w, start%w), |(y, x)| {
        possible_neighbours.iter().filter_map(|(sx, sy)| {
            if x.wrapping_add(*sx) >= w ||y.wrapping_add(*sy) >= h || map[x+sx + w * (y+sy)].saturating_sub(map[x + w * y]) > 1{
                None
            } else {
                Some((y+sy, x+sx))
            }
        }).collect::<Vec<_>>()
    }, |p| *p == (end/w, end%w)).unwrap().len();
    println!("{}", a-1);
}

fn main() {
    part12();
    part1_petgraph();
    part1_pathfinding();
}

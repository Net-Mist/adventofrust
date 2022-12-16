use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const MAX_TIME: u32 = 30;
const MAX_TIME2: u32 = 26;

struct Status {
    time: u32,
    flow: u32,
    position: &'static str,
    opened_valves: HashSet<&'static str>,
}

#[derive(PartialEq, Eq, Clone)]
struct Status2 {
    time: u32,
    time2: u32,
    flow: u32,
    position: &'static str,
    position2: &'static str,
    opened_valves: HashSet<&'static str>,
}

fn data() -> (
    HashMap<&'static str, (u32, Vec<&'static str>)>,
    HashMap<&'static str, usize>,
    Vec<Vec<u32>>,
) {
    let r = Regex::new(r"Valve (.+) has flow rate=(\d+); tunnel.+ to valves? (.+)").unwrap();

    let valves = include_str!("../i.txt")
        .lines()
        .map(|l| {
            let res = r.captures(l).unwrap();
            let valve = res.get(1).unwrap().as_str();
            let flow = res.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let next_valve = res.get(3).unwrap().as_str().split(", ").collect::<Vec<_>>();

            (valve, (flow, next_valve))
        })
        .collect::<HashMap<_, _>>();

    let interesting_valves = valves
        .iter()
        .filter(|(_, (v, _))| *v > 0)
        .map(|(k, _)| *k)
        .enumerate()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();
    let l = interesting_valves.len();
    let mut dist_matrix = vec![vec![0; l]; l];
    for (&v1, &i1) in interesting_valves.iter() {
        for (&v2, &i2) in interesting_valves.iter() {
            if i1 == i2 {
                continue;
            }
            dist_matrix[i1][i2] = move_to(v1, v2, &valves);
        }
    }
    (valves, interesting_valves, dist_matrix)
}

fn move_to(valve: &str, pos: &str, d: &HashMap<&'static str, (u32, Vec<&'static str>)>) -> u32 {
    pathfinding::directed::bfs::bfs(
        &valve,
        |valve| {
            let a = d.get(valve);
            let b = a.unwrap().1.clone();
            b
        },
        |p| *p == pos,
    )
    .unwrap()
    .len() as u32
        - 1
}

fn part1() {
    let (valves, interesting_valves, dist_matrix) = data();

    let mut status = vec![Status {
        time: 0,
        flow: 0,
        position: "AA",
        opened_valves: HashSet::new(),
    }];
    let mut max_flow = 0;

    while !status.is_empty() {
        let cur_status = status.pop().unwrap();

        for (&v, &i) in interesting_valves.iter() {
            if cur_status.opened_valves.contains(v) {
                continue;
            }

            let t = if interesting_valves.contains_key(&cur_status.position) {
                dist_matrix[i][*interesting_valves.get(&cur_status.position).unwrap()] + 1
            } else {
                move_to(v, cur_status.position, &valves) + 1
            };

            if cur_status.time + t >= MAX_TIME {
                continue;
            }

            let mut opened_valves = cur_status.opened_valves.clone();
            opened_valves.insert(&v);
            let mut flow =
                cur_status.flow + (MAX_TIME - (cur_status.time + t)) * valves.get(&v).unwrap().0;
            max_flow = max_flow.max(flow);
            status.push(Status {
                time: cur_status.time + t,
                flow: flow,
                position: v,
                opened_valves: opened_valves,
            })
        }
    }

    println!("{max_flow}");
}

fn part2() {
    let (valves, interesting_valves, dist_matrix) = data();

    let mut status = vec![Status2 {
        time: 0,
        time2: 0,
        flow: 0,
        position: "AA",
        position2: "AA",
        opened_valves: HashSet::new(),
    }];
    let mut max_flow = 0;

    while !status.is_empty() {
        println!("{}, {}", status.len(), max_flow);
        let cur_status = status.pop().unwrap();
        println!("{} {}", cur_status.time, cur_status.time2);

        for (&v, &i) in interesting_valves.iter() {
            if cur_status.opened_valves.contains(v) {
                continue;
            }
            // Us
            let t = if interesting_valves.contains_key(&cur_status.position) {
                dist_matrix[i][*interesting_valves.get(&cur_status.position).unwrap()] + 1
            } else {
                move_to(v, cur_status.position, &valves) + 1
            };

            if cur_status.time + t > MAX_TIME2 {
                continue;
            }

            let mut opened_valves = cur_status.opened_valves.clone();
            opened_valves.insert(&v);
            let flow =
                cur_status.flow + (MAX_TIME2 - (cur_status.time + t)) * valves.get(&v).unwrap().0;
            max_flow = max_flow.max(flow);
            let s = Status2 {
                time: cur_status.time + t,
                time2: cur_status.time2,
                flow: flow,
                position: v,
                position2: cur_status.position2,
                opened_valves: opened_valves,
            };

            for (&v2, &i2) in interesting_valves.iter() {
                if v == v2 {
                    continue;
                }
                if cur_status.opened_valves.contains(v2) {
                    continue;
                }

                // Elephant
                let i = i2;
                let v = v2;
                let t = if interesting_valves.contains_key(&cur_status.position2) {
                    dist_matrix[i][*interesting_valves.get(&cur_status.position2).unwrap()] + 1
                } else {
                    move_to(v, cur_status.position2, &valves) + 1
                };

                if cur_status.time2 + t < MAX_TIME2 {
                    let mut opened_valves = s.opened_valves.clone();
                    opened_valves.insert(&v);
                    let flow = s.flow + (MAX_TIME2 - (s.time2 + t)) * valves.get(&v).unwrap().0;
                    max_flow = max_flow.max(flow);
                    status.push(Status2 {
                        time: s.time,
                        time2: s.time2 + t,
                        flow,
                        position: s.position,
                        position2: v,
                        opened_valves: opened_valves,
                    })
                }
            }
        }
    }

    println!("{max_flow}");
}

fn main() {
    // part1();
    part2();
}

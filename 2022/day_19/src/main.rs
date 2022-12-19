use regex::Regex;

use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    geode_robot: u32,
    geode: u32,
    obsidian_robot: u32,
    obsidian: u32,
    clay_robot: u32,
    clay: u32,
    ore_robot: u32,
    ore: u32,
}

fn insert(s: &State, all_states: &mut HashSet<State>, queue: &mut BinaryHeap<State>) {
    if all_states.contains(s) {
        return;
    }
    all_states.insert(*s);
    queue.push(*s)
}

fn data(part2: bool) -> Vec<(u32, u32, u32, u32, u32, u32, u32)> {
    let r = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let taken = if part2 { 3 } else { usize::MAX };
    include_str!("../i.txt")
        .lines()
        .map(|line| {
            let res = r.captures(line).unwrap();
            (
                res.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                res.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                res.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                res.get(4).unwrap().as_str().parse::<u32>().unwrap(),
                res.get(5).unwrap().as_str().parse::<u32>().unwrap(),
                res.get(6).unwrap().as_str().parse::<u32>().unwrap(),
                res.get(7).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .take(taken)
        .collect()
}

fn part(part2: bool) {
    let d = data(part2);

    let mut score1 = 0;
    let mut score2 = 1;
    for blueprint in d.into_iter() {
        let id = blueprint.0;
        let s = State {
            ore_robot: 1, ..Default::default()
        };
        let mut to_explore = BinaryHeap::new();
        to_explore.push(s);
        let mut all_states = HashSet::new();
        all_states.insert(s);
        let n_steps = if part2 { 32 } else { 24 };
        for _ in 0..n_steps {
            let mut to_explore_new = BinaryHeap::new();
            let mut all_states_new = HashSet::new();

            while !to_explore.is_empty() {
                let s = to_explore.pop().unwrap();

                // generate ressources
                let mut new_s = s;
                new_s.ore += s.ore_robot;
                new_s.clay += s.clay_robot;
                new_s.obsidian += s.obsidian_robot;
                new_s.geode += s.geode_robot;

                // build robot
                if s.ore >= blueprint.5 && s.obsidian >= blueprint.6 {
                    let mut new_s = new_s;
                    new_s.geode_robot += 1;
                    new_s.ore -= blueprint.5;
                    new_s.obsidian -= blueprint.6;
                    insert(&new_s, &mut all_states_new, &mut to_explore_new);
                }
                if s.ore >= blueprint.3 && s.clay >= blueprint.4 {
                    let mut new_s = new_s;
                    new_s.obsidian_robot += 1;
                    new_s.ore -= blueprint.3;
                    new_s.clay -= blueprint.4;
                    insert(&new_s, &mut all_states_new, &mut to_explore_new);
                }
                if s.ore >= blueprint.2 {
                    let mut new_s = new_s;
                    new_s.clay_robot += 1;
                    new_s.ore -= blueprint.2;
                    insert(&new_s, &mut all_states_new, &mut to_explore_new);
                }
                if s.ore >= blueprint.1 {
                    let mut new_s = new_s;
                    new_s.ore_robot += 1;
                    new_s.ore -= blueprint.1;
                    insert(&new_s, &mut all_states_new, &mut to_explore_new);
                }

                // do nothing
                insert(&new_s, &mut all_states_new, &mut to_explore_new);
            }

            to_explore = BinaryHeap::new();
            for _ in 0..100.min(to_explore_new.len()) {
                to_explore.push(to_explore_new.pop().unwrap());
            }
        }
        let mut max_geode = 0;

        for s in to_explore.iter() {
            max_geode = max_geode.max(s.geode);
            continue;
        }

        score2 *= max_geode;
        score1 += id * max_geode;
    }
    if part2 {
        println!("{:?}", score2);
    } else {
        println!("{:?}", score1);
    }
}

fn main() {
    part(false);
    part(true);
}

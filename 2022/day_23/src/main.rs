use std::collections::hash_map::Entry;
use std::collections::HashMap;

static TO_CHECK: [[(i32, i32); 3]; 4] =
    [[(-1, -1), (0, -1), (1, -1)], [(-1, 1), (0, 1), (1, 1)], [(-1, -1), (-1, 0), (-1, 1)], [(1, -1), (1, 0), (1, 1)]];

fn data() -> HashMap<(i32, i32), i32> {
    let v = include_str!("../i.txt")
        .lines()
        .map(|line| line.chars().enumerate().filter(|(_, c)| *c == '#').map(|(i, _)| i).collect::<Vec<_>>())
        .enumerate()
        .collect::<Vec<_>>();

    let mut elf_position = HashMap::new();

    let mut i = 0;
    for (l, vc) in v.into_iter() {
        for c in vc {
            elf_position.insert((c as i32, l as i32), i);
            i += 1;
        }
    }

    elf_position
}

fn part1(r: usize) {
    let mut elf_position = data();
    let mut elf_to_position = vec![(0i32, 0i32); elf_position.len()];
    for (k, v) in elf_position.iter() {
        elf_to_position[*v as usize] = *k
    }

    let mut check_id = 0;
    for r in 1..=r {
        let mut new_position = HashMap::new();

        for (position, i) in elf_position.iter() {
            // check neighbor
            if (-1..=1).all(|i| {
                (-1..=1).all(|j| i == 0 && j == 0 || !elf_position.contains_key(&(position.0 + i, position.1 + j)))
            }) {
                new_position.insert(*position, *i);
                continue;
            }

            // find direction
            let id = (0..4).map(|j| (check_id + j as i32).rem_euclid(4i32)).find(|j| {
                TO_CHECK[*j as usize].iter().all(|d| !elf_position.contains_key(&(d.0 + position.0, d.1 + position.1)))
            });
            if let Some(j) = id {
                let np = (TO_CHECK[j as usize][1].0 + position.0, TO_CHECK[j as usize][1].1 + position.1);
                match new_position.entry(np) {
                    Entry::Occupied(mut e) => {
                        e.insert(-1);
                    }
                    Entry::Vacant(e) => {
                        e.insert(*i);
                    }
                }
            }
        }

        let mut no_move = true;
        for (k, v) in new_position {
            if v == -1 {
                continue;
            }
            let old_position = elf_to_position[v as usize];
            if elf_position.get(&old_position) == Some(&v) {
                elf_position.remove(&old_position);
            }
            elf_to_position[v as usize] = k;
            elf_position.insert(k, v);
            if old_position != k {
                no_move = false
            }
        }
        check_id += 1;
        if no_move {
            println!("end {r}");
            break;
        }
    }

    let m = elf_to_position.iter().fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |acc, e| {
        (acc.0.min(e.0), acc.1.max(e.0), acc.2.min(e.1), acc.3.max(e.1))
    });

    println!("area: {}", (m.1 - m.0 + 1) * (m.3 - m.2 + 1) - elf_position.len() as i32);
}

fn main() {
    part1(10);
    part1(10000);
}

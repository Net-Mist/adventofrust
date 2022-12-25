use std::collections::HashSet;

static MOVE: [(i32, i32); 5] = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)];

type Map = Vec<Vec<Vec<char>>>;

fn data() -> Map {
    include_str!("../i.txt").lines().map(|line| line.chars().map(|c| vec![c]).collect()).collect()
}

fn iter_neighbors(p: (i32, i32), map: &Map) -> Vec<(i32, i32)> {
    MOVE.iter()
        .map(|&v| (p.0 + v.0, p.1 + v.1))
        .filter(|&v| {
            (v.0 >= 1 && v.0 < map.len() as i32 - 1 && v.1 >= 1 && v.1 < map[0].len() as i32 - 1)
                || v == (0, 1)
                || v == (map.len() as i32 - 1, map[0].len() as i32 - 2)
        })
        .collect()
}

fn part12() {
    let mut map = data();
    let h = map.len();
    let w = map[0].len();

    let position = (0, map[0].iter().position(|p| p[0] == '.').unwrap() as i32);
    let goal = ((h - 1) as i32, map[h - 1].iter().position(|p| p[0] == '.').unwrap() as i32);
    let goals = vec![goal, position, goal];
    let mut goal_id = 0;

    let mut all_positions = HashSet::new();
    all_positions.insert(position);

    for i in 1..300000000 {
        // move blizzard
        let mut new_map = vec![vec![Vec::<char>::new(); w]; h];
        for y in 0..h {
            for x in 0..w {
                for b in map[y][x].iter() {
                    match b {
                        '>' => new_map[y][x % (w - 2) + 1].push('>'),
                        '<' => new_map[y][((x as i32 - 2).rem_euclid((w - 2) as i32) + 1) as usize].push('<'),
                        '^' => new_map[((y as i32 - 2).rem_euclid((h - 2) as i32) + 1) as usize][x].push('^'),
                        'v' => new_map[y % (h - 2) + 1][x].push('v'),
                        _ => {}
                    }
                }
            }
        }
        map = new_map;

        // move player
        let mut new_all_positions = HashSet::new();
        for p in all_positions {
            for n in iter_neighbors(p, &map) {
                if map[n.0 as usize][n.1 as usize].is_empty() {
                    new_all_positions.insert(n);
                }
            }
        }
        all_positions = new_all_positions;

        if all_positions.contains(&goals[goal_id]) {
            all_positions.clear();
            all_positions.insert(goals[goal_id]);
            goal_id += 1;
            if goals.len() == goal_id {
                println!("end in {:?}", i);
                return;
            }
            println!("phase {goal_id} ends in {i}");
        }
    }
}

fn main() {
    part12();
}

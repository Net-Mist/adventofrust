use std::collections::HashMap;
use std::collections::VecDeque;

const N_ROCKS1: u128 = 2022;
const N_ROCKS2: u128 = 1000000000000;

fn data() -> Vec<&'static u8> {
    include_bytes!("../i.txt")
        .iter()
        .filter(|&&b| b == b'<' || b == b'>')
        .collect()
}

fn check_move(
    shape: &Vec<(i32, i32)>,
    map: &Vec<Vec<i32>>,
    d: &(i32, i32),
    position: &(i32, i32),
) -> bool {
    for (bx, by) in shape.iter() {
        let x = bx + d.0 + position.0;
        let y = by + d.1 + position.1;

        if !(0..=6).contains(&x) || y < 0 {
            return false;
        }
        if map[y as usize][x as usize] != 0 {
            return false;
        }
    }
    true
}

fn part1() {
    let d = data();

    // relative position from lower left (dx, dy)
    let rel_shape = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut cur_shape = 0;
    let mut max_y = 0;
    // map is build bottom up
    let mut map = vec![vec![0; 7]; 5000];
    let mut gaz_action = 0;

    for _ in 0..2022 {
        let shape = rel_shape.get(cur_shape).unwrap();
        cur_shape = (cur_shape + 1) % 5;
        let mut pos = (2, max_y + 3);

        loop {
            // gaz action
            let m = match d.get(gaz_action).unwrap() {
                b'<' => -1,
                b'>' => 1,
                _ => unreachable!(),
            };
            gaz_action = (gaz_action + 1) % d.len();
            if check_move(shape, &map, &(m, 0), &pos) {
                pos.0 += m;
            }

            // gravity action
            if check_move(shape, &map, &(0, -1), &pos) {
                pos.1 -= 1;
            } else {
                break;
            }
        }

        for (bx, by) in shape.iter() {
            map[(by + pos.1) as usize][(bx + pos.0) as usize] = 1;
            max_y = max_y.max(by + pos.1 + 1)
        }
    }
    println!("final h: {:?}", max_y);
}

fn check_move2(
    shape: &Vec<(i32, i32)>,
    map: &VecDeque<Vec<i32>>,
    d: &(i32, i32),
    position: &(i32, i32),
) -> bool {
    for (bx, by) in shape.iter() {
        let x = bx + d.0 + position.0;
        let y = by + d.1 + position.1;

        if !(0..=6).contains(&x) || y < 0 {
            return false;
        }
        if map[y as usize][x as usize] != 0 {
            return false;
        }
    }
    true
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    map: VecDeque<Vec<i32>>,
    gaz_action: usize,
    cur_shape: usize,
}

fn part2(n_rocks: u128) {
    let gaz = data();

    // relative position from lower left (dx, dy)
    let rel_shape = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut cur_shape = 0;
    let mut real_max_y: u128 = 0;
    let mut real_i = 0;
    let mut max_y = 0;
    // map is build bottom up
    let mut map: VecDeque<Vec<i32>> = vec![vec![0; 7]; 100].into_iter().collect();
    let mut gaz_action = 0;

    let mut hashed_steps = HashMap::new();

    for i in 0..n_rocks {
        let shape = rel_shape.get(cur_shape).unwrap();
        cur_shape = (cur_shape + 1) % rel_shape.len();
        let mut pos = (2, max_y + 3);

        loop {
            // gaz action
            let m = match gaz.get(gaz_action).unwrap() {
                b'<' => -1,
                b'>' => 1,
                _ => unreachable!(),
            };
            gaz_action = (gaz_action + 1) % gaz.len();
            if check_move2(shape, &map, &(m, 0), &pos) {
                pos.0 += m;
            }
            // gravity action
            if check_move2(shape, &map, &(0, -1), &pos) {
                pos.1 -= 1;
            } else {
                break;
            }
        }

        let max_y_origin = max_y;
        for (bx, by) in shape.iter() {
            map[(by + pos.1) as usize][(bx + pos.0) as usize] = 1;
            max_y = max_y.max(by + pos.1 + 1);
        }
        real_max_y += (max_y as u128) - max_y_origin as u128;

        // adapt map
        if max_y > 90 {
            let d = max_y - 90;
            for _ in 0..d {
                map.pop_front();
                map.push_back(vec![0; 7]);
            }
            max_y -= d;
        }

        let s = State {
            map: map.clone(),
            gaz_action,
            cur_shape,
        };
        if hashed_steps.contains_key(&s) && real_i == 0 {
            println!("yeah jump in the future!");
            let (previous_h, previous_i) = hashed_steps.get(&s).unwrap();
            let di = i - previous_i;
            let dh = real_max_y - previous_h;
            let n_steps = (n_rocks - i) / di;
            real_max_y += n_steps * dh;
            real_i = n_steps * di;
            println!("{} {}, {}, {}", di, dh, real_i, n_steps);
            println!("{} {}", i, previous_i);
        }
        hashed_steps.insert(s, (real_max_y, i));

        if i + real_i == n_rocks {
            break;
        }
    }
    println!("final h: {:?}", real_max_y-1);
}

fn main() {
    part1();
    part2(N_ROCKS2);
}

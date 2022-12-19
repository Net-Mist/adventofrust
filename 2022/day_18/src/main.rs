use pathfinding::prelude::astar;
use std::collections::HashSet;

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
    include_str!("../i2.txt")
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

fn part1() {
    let d = data();
    println!("{:?}", d);

    let mut n_faces = 0;
    for (x, y, z) in d.iter() {
        if !d.contains(&(*x, *y, z + 1)) {
            n_faces += 1
        }
        if !d.contains(&(*x, *y, z - 1)) {
            n_faces += 1
        }
        if !d.contains(&(*x, y + 1, *z)) {
            n_faces += 1
        }
        if !d.contains(&(*x, y - 1, *z)) {
            n_faces += 1
        }
        if !d.contains(&(x + 1, *y, *z)) {
            n_faces += 1
        }
        if !d.contains(&(x - 1, *y, *z)) {
            n_faces += 1
        }
    }
    println!("{:?}", n_faces);
}

// fn find_duplicate(d: &HashSet<Point>)  -> HashSet<(i32,i32)>{

// }

fn add_neighbour_to_set(point: &Point, point_to_consider: &HashSet<Point>, s: &mut HashSet<Point>) {
    for v in NEIGHBORED.iter() {
        let n_p = (point.0 + v.0, point.1 + v.1, point.2 + v.2);
        if point_to_consider.contains(&n_p) && !s.contains(&n_p) {
            s.insert(n_p);
            add_neighbour_to_set(&n_p, point_to_consider, s);
        }
    }
}

fn part2() {
    let mut d = data();
    println!("{:?}", d);

    let mut duplicate_elements_x_y = HashSet::new();
    let mut single_elements = HashSet::new();
    for e in d.iter().map(|p| (p.0, p.1)) {
        if single_elements.contains(&e) {
            duplicate_elements_x_y.insert(e)
        } else {
            single_elements.insert(e)
        };
    }

    let mut duplicate_elements_x_z = HashSet::new();
    let mut single_elements = HashSet::new();
    for e in d.iter().map(|p| (p.0, p.2)) {
        if single_elements.contains(&e) {
            duplicate_elements_x_z.insert(e)
        } else {
            single_elements.insert(e)
        };
    }

    let mut duplicate_elements_y_z = HashSet::new();
    let mut single_elements = HashSet::new();
    for e in d.iter().map(|p| (p.1, p.2)) {
        if single_elements.contains(&e) {
            duplicate_elements_y_z.insert(e)
        } else {
            single_elements.insert(e)
        };
    }

    let mut maybe_inside = HashSet::new();
    for e in duplicate_elements_x_y.iter() {
        let interesting_droplet = d.iter().filter(|p| p.0 == e.0 && p.1 == e.1);
        let mut min_z = i32::MAX;
        let mut max_z = i32::MIN;
        for d in interesting_droplet {
            min_z = min_z.min(d.2);
            max_z = max_z.max(d.2);
        }
        for z in min_z + 1..max_z {
            if duplicate_elements_x_z.contains(&(e.0, z))
                && duplicate_elements_y_z.contains(&(e.1, z))
                && !d.contains(&(e.0, e.1, z))
            {
                maybe_inside.insert((e.0, e.1, z));
            }
        }
    }

    println!("{:?} points maybe inside", maybe_inside.len());

    let mut not_inside_for_sure = HashSet::new();
    for p in maybe_inside.iter() {
        // println!("{:?}", p);
        if not_inside_for_sure.contains(p) || d.contains(p) {
            continue;
        }
        let a = astar(
            p,
            |&(y, x, z)| {
                NEIGHBORED
                    .iter()
                    .map(|&v| ((x + v.0, y + v.1, z + v.2), 1))
                    .filter(|(p, _)| !d.contains(p))
                    .collect::<Vec<_>>()
            },
            |&(x, y, z)| (x.abs_diff(-1) + y.abs_diff(-1) + z.abs_diff(-1)) / 3,
            |&p| p == (-1, -1, -1),
        );
        match a {
            None => {
                println!("add {:?} to the list of points", p);
                d.insert(*p);
                add_neighbour_to_set(p, &maybe_inside, &mut d);
            }
            _ => {
                println!("add {:?} to the list of points not interesting", p);
                not_inside_for_sure.insert(*p);
                add_neighbour_to_set(p, &maybe_inside, &mut not_inside_for_sure);
            }
        };
    }
    println!("{}", d.len());

    let mut n_faces = 0;
    for (x, y, z) in d.iter() {
        if !d.contains(&(*x, *y, z + 1)) {
            n_faces += 1
        }
        if !d.contains(&(*x, *y, z - 1)) {
            n_faces += 1
        }
        if !d.contains(&(*x, y + 1, *z)) {
            n_faces += 1
        }
        if !d.contains(&(*x, y - 1, *z)) {
            n_faces += 1
        }
        if !d.contains(&(x + 1, *y, *z)) {
            n_faces += 1
        }
        if !d.contains(&(x - 1, *y, *z)) {
            n_faces += 1
        }
    }
    println!("{:?}", n_faces);
}

fn main() {
    // part1();
    part2();
}

// 4256
// 4284 too high

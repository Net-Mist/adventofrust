use std::collections::HashSet;
use regex::Regex;

fn data() -> Vec<((i64, i64), (i64, i64), u64)> {
    let r = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    include_str!("../i.txt")
        .lines()
        .map(|l| {
            let res = r.captures(l).unwrap();
            let sensor_x = res.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let sensor_y = res.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let beacon_x = res.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let beacon_y = res.get(4).unwrap().as_str().parse::<i64>().unwrap();

            (
                (sensor_x, sensor_y),
                (beacon_x, beacon_y),
                (sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y)),
            )
        })
        .collect()
}

fn part1() {
    let d = data();
    let r = 2000000;
    let mut h = HashSet::new();
    let mut h_b = HashSet::new();

    for ((sensor_x, sensor_y), (beacon_x, beacon_y), dist) in d.into_iter() {
        if beacon_y == r {
            h_b.insert(beacon_x);
        }
        if sensor_y.abs_diff(r) <= dist {
            let a = (dist - sensor_y.abs_diff(r)) as i64;
            for x in sensor_x - a..sensor_x + a + 1 {
                h.insert(x);
            }
        }
    }
    println!("{:?}", h.len() - h_b.len());
}

fn get_limit_points(p: (i64, i64), d: u64) -> Vec<(i64, i64)> {
    let mut out = Vec::new();
    for i in 0..d + 1 {
        let d = d as i64;
        let i = i as i64;
        out.push((p.0 - d + i - 1, p.1 + i));
        out.push((p.0 + d - i + 1, p.1 - i));
        out.push((p.0 - i, p.1 - d + i - 1));
        out.push((p.0 + i, p.1 + d - i + 1));
    }
    out
}

fn compute_dist(p: (i64, i64), p2: (i64, i64)) -> u64 {
    p.0.abs_diff(p2.0) + p.1.abs_diff(p2.1)
}

#[test]
fn test_get_limit_points() {
    println!("{:?}", get_limit_points((0, 0), 1));
    assert_eq!(
        get_limit_points((0, 0), 1),
        vec![
            (-2, 0),
            (2, 0),
            (0, -2),
            (0, 2),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 1)
        ]
    )
}

fn part2() {
    let d = data();

    let max_x = 4000000;
    let max_y = 4000000;

    for ((sensor_x, sensor_y), (_beacon_x, _beacon_y), dist) in d.iter() {
        let ps = get_limit_points((*sensor_x, *sensor_y), *dist);
        'outer: for p in ps.iter() {
            if p.0 >= 0 && p.0 <= max_x && p.1 >= 0 && p.1 <= max_y {
                for ((sensor_x2, sensor_y2), _, d2) in d.iter() {
                    if compute_dist((*sensor_x2, *sensor_y2), *p) <= *d2 {
                        continue 'outer;
                    }
                }
                println!("{}", p.0 * 4000000 + p.1);
                return;
            }
        }
    }
}

fn main() {
    part1();
    part2();
}

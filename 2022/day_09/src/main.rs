use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn data() -> impl Iterator< Item = (char, u32)> {
    include_str!("../i.txt")
        .lines()
        .map(|l| {
            let (m, n) = l.split_once(' ').unwrap();
            (m.chars().next().unwrap(), n.parse::<u32>().unwrap())
        })
}

fn new_t(h: Pos, t: Pos) -> Pos {
    let mut t = t.clone();
    if h.y == t.y {
        if h.x - t.x > 1 {
            t.x += 1;
        } else if h.x - t.x < -1 {
            t.x -= 1;
        }
    } else if h.x == t.x {
        if h.y - t.y > 1 {
            t.y += 1;
        } else if h.y - t.y < -1 {
            t.y -= 1;
        }
    } else {
        // diag move
        if t.x.abs_diff(h.x) == 1 && t.y.abs_diff(h.y) == 1 {
            return t;
        }

        if h.x - t.x > 0 {
            t.x += 1;
        } else {
            t.x -= 1;
        }
        if h.y - t.y > 0 {
            t.y += 1;
        } else {
            t.y -= 1;
        }
    }
    t
}

fn part12() {
    let mut r = [Pos { x: 0, y: 0 }; 10];
    let mut all_pos1 = HashSet::<Pos>::new();
    let mut all_pos2 = HashSet::<Pos>::new();
    all_pos1.insert(r[1]);
    all_pos2.insert(r[9]);

    for (m, n) in data() {
        for _ in 0..n {
            match m {
                'R' => {r[0].x += 1},
                'U' => {r[0].y += 1},
                'L' => {r[0].x -= 1},
                'D' => {r[0].y -= 1},
                _ => unreachable!(),
            }
            for i in 0..9 {
                let t = new_t(r[i], r[i + 1]);
                r[i+1] = t;
            }
            all_pos1.insert(r[1]);
            all_pos2.insert(r[9]);
        }
    }

    println!("{}", all_pos1.len());
    println!("{}", all_pos2.len());
}

fn main() {
    part12();
}

fn data(part_2: bool) -> (Vec<Vec<usize>>, usize, usize, usize, usize) {
    let mut coords: Vec<Vec<(usize, usize)>> = include_str!("../i.txt")
        .lines()
        .map(|line| {
            line.split(" -> ")
                .into_iter()
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut min_x = coords
        .iter()
        .map(|l| l.iter().map(|p| p.0).min().unwrap())
        .min()
        .unwrap()
        - 1;
    let min_y = 0;
    let mut max_x = coords
        .iter()
        .map(|l| l.iter().map(|p| p.0).max().unwrap())
        .max()
        .unwrap()
        + 1;
    let max_y = coords
        .iter()
        .map(|l| l.iter().map(|p| p.1).max().unwrap())
        .max()
        .unwrap()
        + 2;

    min_x = min_x.min(500 - max_y - 3);
    max_x = max_x.max(500 + max_y + 3);

    let mut map = vec![vec![0; max_x - min_x + 1]; max_y - min_y + 1];

    if part_2 {
        coords.push(vec![(min_x, max_y), (max_x, max_y)]);
    }

    for mut line in coords.into_iter() {
        let mut p = line.pop().unwrap();
        for p1 in line.into_iter().rev() {
            if p.0 == p1.0 {
                let y0 = p.1.min(p1.1);
                let y1 = p.1.max(p1.1);
                for y in y0..(y1 + 1) {
                    map[y - min_y][p.0 - min_x] = 1;
                }
            }
            if p.1 == p1.1 {
                let x0 = p.0.min(p1.0);
                let x1 = p.0.max(p1.0);
                for x in x0..(x1 + 1) {
                    map[p.1 - min_y][x - min_x] = 1;
                }
            }
            p = p1;
        }
    }

    (map, min_x, min_y, max_x, max_y)
}

fn part(part2: bool) {
    let (mut map, min_x, min_y, _, max_y) = data(part2);
    let mut i = 0;

    loop {
        if map[0][500 - min_x] != 0 {
            println!("part2: {}", i);
            return;
        }

        i += 1;
        let (mut s_x, mut s_y) = (500, 0);
        loop {
            if s_y == max_y {
                // then fell forever
                println!("part1: {}", i - 1);
                return;
            } else if map[s_y + 1 - min_y][s_x - min_x] == 0 {
                s_y += 1;
            } else if map[s_y + 1 - min_y][s_x - 1 - min_x] == 0 {
                s_y += 1;
                s_x -= 1;
            } else if map[s_y + 1 - min_y][s_x + 1 - min_x] == 0 {
                s_y += 1;
                s_x += 1;
            } else {
                // can't move anymore
                map[s_y - min_y][s_x - min_x] = 2;
                break;
            }
        }
    }
}

fn main() {
    part(false);
    part(true);
}

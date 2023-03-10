fn data() -> Vec<Vec<u32>> {
    include_str!("../i.txt")
        .lines()
        .map(|b| {
            b.chars()
                .map(|n| n.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn update_map(i: usize, j: usize, max: &mut u32, map: &[Vec<u32>], b_map: &mut [Vec<bool>]) {
    if map[i][j] > *max {
        *max = map[i][j];
        if !b_map[i][j] {
            b_map[i][j] = true;
        }
    }
}

fn part1() {
    let d = data();
    let size = d.len();
    let mut visible_map = vec![vec![false; size]; size];
    for i in 0..size {
        visible_map[i][0] = true;
        visible_map[0][i] = true;
        visible_map[i][size - 1] = true;
        visible_map[size - 1][i] = true;
    }

    for i in 0..size {
        let mut max_up = d[0][i];
        let mut max_down = d[size - 1][i];
        let mut max_left = d[i][0];
        let mut max_right = d[i][size - 1];
        for j in 1..size {
            update_map(j, i, &mut max_up, &d, &mut visible_map);
            update_map(size - 1 - j, i, &mut max_down, &d, &mut visible_map);
            update_map(i, j, &mut max_left, &d, &mut visible_map);
            update_map(i, size - 1 - j, &mut max_right, &d, &mut visible_map);
        }
    }

    println!(
        "{}",
        visible_map
            .into_iter()
            .map(|v| v.into_iter().map(|i| i as u32).sum::<u32>())
            .sum::<u32>()
    );
}


fn part2() {
    let d = data();
    let size = d.len();

    let mut max_d = 0;
    for i in 0..size {
        for j in 0..size {
            let h = d[i][j];

            let (mut up, mut down, mut l, mut r) = (0,0,0,0);

            for x in (0..j).rev() {
                l += 1;
                if h <= d[i][x] {
                    break;
                }
            }
            for x in j+1..size {
                r += 1;
                if h <= d[i][x] {
                    break;
                }
            }
            for y in (0..i).rev() {
                up += 1;
                if h <= d[y][j] {
                    break;
                }
            }
            for y in i+1..size {
                down += 1;
                if h <= d[y][j] {
                    break;
                }
            }

            max_d = max_d.max(up*down*l*r);
        }
    }
    println!("{}", max_d);
}

fn main() {
    part1();
    part2();
}

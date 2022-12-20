fn part1() {
    let mut d = include_str!("../i.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();
    let l = d.len();

    for i in 0..l {
        let r_i = d.iter().position(|(init_i, _)| *init_i == i).unwrap();
        let v = d.remove(r_i).1;
        let new_i = (r_i as i32 + v).rem_euclid(l as i32 - 1);
        d.insert(new_i as usize, (i, v));
    }
    let p_0 = d.iter().position(|(_init_i, v)| *v == 0).unwrap();
    println!(
        "{}",
        (1..4)
            .map(|i| d.get((p_0 + 1000 * i) % l).unwrap().1)
            .sum::<i32>()
    );
}

fn part2() {
    let mut d = include_str!("../i.txt")
        .lines()
        .map(|l| l.parse::<i128>().unwrap() * 811589153)
        .enumerate()
        .collect::<Vec<_>>();
    let l = d.len();

    for _ in 0..10 {
        for i in 0..l {
            let r_i = d.iter().position(|(init_i, _v)| *init_i == i).unwrap();
            let v = d.remove(r_i).1;
            let new_i = (r_i as i128 + v + (l as i128 - 1)).rem_euclid(l as i128 - 1);
            d.insert(new_i as usize, (i, v));
        }
    }

    let p_0 = d.iter().position(|(_init_i, v)| *v == 0).unwrap();
    println!(
        "{}",
        (1..4)
            .map(|i| d.get((p_0 + 1000 * i) % l).unwrap().1)
            .sum::<i128>()
    );
}

fn main() {
    part1();
    part2();
}

use std::collections::VecDeque;

fn main() {
    let mut a = include_str!("../i.txt")
        .lines()
        .map(|line| {
            line.chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    let i = 5i64.pow(i as u32);
                    match c {
                        '0' => 0,
                        '1' => i,
                        '2' => 2 * i,
                        '-' => -i,
                        '=' => -2 * i,
                        _ => unreachable!(),
                    }
                })
                .sum::<i64>()
        }).sum::<i64>();
    
    let mut out = VecDeque::new();
    while a > 0 {
        a += 2;
        let r = a.rem_euclid(5);
        a /= 5;

        let c = match r {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!(),
        };
        out.push_front(c);
    }
    for c in out {
        print!("{c}")
    }
    println!()
}

use std::collections::HashSet;

fn char_to_prio(i: char) -> u32 {
    match i.is_ascii_lowercase() {
        false => (i as u32) - 64 + 26,
        true => (i as u32) - 96,
    }
}

#[test]
fn test_char_to_prio() {
    assert_eq!(char_to_prio('a'), 1);
    assert_eq!(char_to_prio('A'), 27);
}

fn part1() {
    let a: u32 = include_str!("../i.txt")
        .lines()
        .map(|b| {
            let l = b.len() / 2;
            let set1 = b.chars().take(l).collect::<HashSet<_>>();
            let set2 = b.chars().rev().take(l).collect();
            char_to_prio(*set1.intersection(&set2).next().unwrap())
        })
        .sum();
    println!("{}", a);
}

fn part2() {
    let a = include_str!("../i.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|l| {
            let set1 = l[0].chars().collect::<HashSet<_>>();
            let set2 = l[1].chars().collect();
            let set3 = l[2].chars().collect::<HashSet<_>>();
            let inter = set1.intersection(&set2).copied().collect();
            let mut inter = set3.intersection(&inter);
            let inter = inter.next().unwrap();
            char_to_prio(*inter)
        })
        .sum::<u32>();
    println!("{}", a);
}

fn main() {
    part1();
    part2();
}

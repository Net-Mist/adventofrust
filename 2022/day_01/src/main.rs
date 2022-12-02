use std::collections::BinaryHeap;

fn data() -> BinaryHeap<u32> {
    include_str!("../i.txt")
        .split("\n\n")
        .map(|b| b.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .collect()
}

fn part1() {
    println!("{}", data().pop().unwrap());
}

fn part2() {
    let mut d = data();
    let mut r = 0;
    for _ in 0..3 {
        r += d.pop().unwrap();
    }
    println!("{r}");

    println!("{}", data().into_iter().take(3).sum::<u32>());
}

fn main() {
    part1();
    part2();
}

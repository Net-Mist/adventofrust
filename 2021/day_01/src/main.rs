fn data() -> Vec<i32> {
    include_str!("../i.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
}

fn part1() {
    let a = data().windows(2).filter(|i| i[0] < i[1]).count();
    println!("{}", a);
}

fn part2() {
    let a = data()
        .windows(3)
        .map(|i| i[0] + i[1] + i[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|i| i[0] < i[1])
        .count();
    println!("{}", a);
}

fn part2bis() {
    let a = data().windows(4).filter(|i| i[0] < i[3]).count();
    println!("{}", a);
}

fn main() {
    part1();
    part2();
    part2bis();
}

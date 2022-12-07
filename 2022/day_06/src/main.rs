use std::collections::HashSet;

fn part(w: usize) {
    let vec = include_str!("../i.txt").chars().collect::<Vec<_>>();
    let a = vec
        .windows(w)
        .enumerate()
        .find(|(_, s)| s.iter().collect::<HashSet<_>>().len() == w);
    println!("{:?}", a.unwrap().0 + w)
}

fn main() {
    part(4);
    part(14);
}

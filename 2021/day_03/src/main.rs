use counter::Counter;

fn data() -> Vec<Vec<usize>> {
    include_str!("../i.txt")
        .lines()
        .map(|line| line.chars().map(|s| s.to_digit(10).unwrap() as usize).collect())
        .collect()
}

fn part1() {
    let d = data();
    let n_words = d.len();
    let binary_gamma_rate = data()
        .into_iter()
        .reduce(|accum, item| accum.into_iter().zip(item.into_iter()).map(|(a, b)| a + b).collect())
        .unwrap();
    let binary_gamma_rate = binary_gamma_rate
        .into_iter()
        .map(|v| (v >= n_words / 2) as usize)
        .collect::<Vec<_>>();

    let gamma_rate: u32 = binary_gamma_rate.iter().copied().fold(0, |i, v| i * 2 + v as u32);
    let epsilon_rate: u32 = binary_gamma_rate.iter().copied().fold(0, |i, v| i * 2 + (1 - v) as u32);

    println!("{:?}", gamma_rate * epsilon_rate);
}

fn part2() {
    let mut d = data();

    let mut i = 0;
    while d.len() > 1 {
        let (mut mc, n) = *d
            .iter()
            .map(|v| v[i])
            .collect::<Counter<_>>()
            .most_common_ordered()
            .first()
            .unwrap();
        if n == d.len() / 2 {
            mc = 1;
        }
        d = d.iter().filter(|v| v[i] == mc).cloned().collect::<Vec<_>>();
        i += 1;
    }
    let oxygen = d[0].iter().copied().fold(0, |i, v| i * 2 + v as u32);

    i = 0;
    d = data();
    while d.len() > 1 {
        let (mut mc, n) = *d
            .iter()
            .map(|v| v[i])
            .collect::<Counter<_>>()
            .most_common_ordered()
            .last()
            .unwrap();
        if n as f32 == d.len() as f32 / 2. {
            mc = 0;
        }
        d = d.iter().filter(|v| v[i] == mc).cloned().collect::<Vec<_>>();
        i += 1;
    }
    let co2 = d[0].iter().copied().fold(0, |i, v| i * 2 + v as u32);

    println!("{}", co2 * oxygen);
}

fn data_b() -> (usize, usize, impl Iterator<Item = usize>) {
    let s = include_str!("../i.txt");
    let word_size = s.lines().next().unwrap().chars().count();
    let n_words = s.lines().count();
    let iter = s.lines().map(|line| usize::from_str_radix(line, 2).unwrap());

    (word_size, n_words, iter)
}

fn part1_b() {
    let (word_size, n_words, d) = data_b();
    let a = d.fold(vec![0; word_size], |count: Vec<_>, v| {
        count
            .into_iter()
            .enumerate()
            .map(|(i, va)| va + ((v & (1 << i)) >> i))
            .collect()
    });
    let gamma = a
        .into_iter()
        .enumerate()
        .map(|(i, v)| ((v >= n_words / 2) as usize) << i)
        .sum::<usize>();

    println!("{}", gamma * (!gamma & ((1 << word_size) - 1)))
}

fn main() {
    part1();
    part2();
    part1_b();
}

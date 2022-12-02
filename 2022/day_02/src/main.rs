#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn compute_points(me: Shape, other: Shape) -> i32 {
    let s = me as i32;
    if me == other {
        return s + 3;
    };
    if (me as i32 - other as i32 + 3) % 3 == 1 {
        return s + 6;
    }
    s
}

#[test]
fn test_compute_points() {
    assert_eq!(compute_points(Shape::Rock, Shape::Rock), 4);
    assert_eq!(compute_points(Shape::Rock, Shape::Paper), 1);
    assert_eq!(compute_points(Shape::Rock, Shape::Scissors), 7);
}

fn parse(shape: &str) -> Shape {
    match shape {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => unreachable!(),
    }
}

fn part1() {
    let s = include_str!("../i.txt")
        .lines()
        .map(|l| {
            let (other, me) = l.split_once(' ').unwrap();
            (parse(other), parse(me))
        })
        .fold(0, |score, (other, me)| score + compute_points(me, other));
    println!("{s}");
}

fn part2() {
    let s = include_str!("../i.txt")
        .lines()
        .map(|l| {
            let (other, me) = l.split_once(' ').unwrap();
            let other = parse(other);
            let me = match me {
                "Z" => match &other {
                    Shape::Paper => Shape::Scissors,
                    Shape::Rock => Shape::Paper,
                    Shape::Scissors => Shape::Rock,
                },
                "Y" => other,
                "X" => match &other {
                    Shape::Paper => Shape::Rock,
                    Shape::Rock => Shape::Scissors,
                    Shape::Scissors => Shape::Paper,
                },
                _ => unreachable!(),
            };
            (other, me)
        })
        .fold(0, |score, (other, me)| score + compute_points(me, other));
    println!("{s}");
}

fn main() {
    part1();
    part2();
}

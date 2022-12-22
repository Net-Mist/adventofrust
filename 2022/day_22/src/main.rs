use std::collections::BinaryHeap;

type Map = Vec<Vec<MapTile>>;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until1},
    combinator::map,
    number::complete::float,
    sequence::tuple,
    IResult,
};
use nom::multi::many0;

#[derive(PartialEq, Clone, Copy)]
enum MapTile {
    None,
    Open,
    Wall
}

#[derive(Debug)]
enum Instruction {
    N(u32),
    Clockwise,
    CounterClockwise   
}

static directions: [(i32,i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn num_parser(l: &str) -> IResult<&str, u32> {
    map(float, |x| x as u32)(l)
}


fn parser(b: &'static str) -> IResult<&str, Vec<Instruction>> {
    many0(alt((
        map(num_parser, |r| Instruction::N(r)),
        map(tag("R"), |_| Instruction::Clockwise),
        map(tag("L"), |_| Instruction::CounterClockwise),
    )))(b)
}


fn data() -> (Map, Vec<Instruction>) {
    let (map, instruction) = include_str!("../i.txt")
        .split_once("\n\n").unwrap();

    let map = map.lines().map(|l| {
        l.chars().map(|c| 
            match c {
                ' ' => MapTile::None,
                '.' => MapTile::Open,
                '#' => MapTile::Wall,
                _ => unreachable!()
            }
        ).collect()
    }).collect();

    let instruction = parser(instruction).unwrap().1;

    (map, instruction)
}

fn move_one(position: (i32, i32), d: (i32,i32), map: &Map) -> (i32, i32) {
    let mut p = position;
    
    loop {
        // println!("p: {:?}",p);
        let n_p = (
            (p.0 + d.0).rem_euclid(map.len() as i32), 
            (p.1 + d.1).rem_euclid(map[0].len() as i32)
        );
        // println!("n_p: {:?}",n_p);

        if map[n_p.0 as usize][n_p.1 as usize] == MapTile::Open {
            return n_p
        }
        if map[n_p.0 as usize][n_p.1 as usize] == MapTile::Wall {
            return position;
        }
        p = n_p;
    }
}

fn part1() {
    let (mut map, instructions) = data();

    let H = map.len() as i32;
    let W = map[0].len() as i32;

    // 0 padding
    for i in 0..H {
        let l =  map.get(i as usize).unwrap();
        let d = W - l.len() as i32;
        if d != 0 {
            map[i as usize].append(&mut vec![MapTile::None; d as usize]);
        }

    }

    let x = map
    .get(0)
    .unwrap()
    .into_iter()
    .position(|t| *t == MapTile::Open ).unwrap();
    let mut position = (0, x as i32);
    let mut direction = 0;

    println!("{:?}", position);

    for instruction in instructions {
        match instruction {
            Instruction::Clockwise => direction = ((direction+1) as i32).rem_euclid(4),
            Instruction::CounterClockwise => direction = ((direction-1) as i32).rem_euclid(4),
            Instruction::N(n) => {
                for _ in 0..n {
                    position = move_one(position, directions[direction as usize], &map);
                    println!("position: {:?}",position);

                }
            }
        }
        println!("after {:?}, position: {:?}, direction: {:?}", instruction, position, direction);

    }
    let r = 1000 * (position.0+1) + 4 * (position.1+1) + direction;
    println!("{:?}", r);


}


fn main() {
    part1();
    // part2();
}

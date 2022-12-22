
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

#[derive(PartialEq, Clone, Copy, Debug)]
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

static directions: [(i32,i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // left, bottom, right, top

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

struct SubMap {
    map: Map,
    edges: Vec<&'static str>,
    top: i32,
    right: i32,
}

fn part2() {
    let (mut map, instructions) = data();

    let H = map.len() as i32;
    let W = map[0].len() as i32;

    // split map to 6 small one

    let sub_maps = vec![
        SubMap {
            map: (0..50).map(|i| {
                (50..100).map(|j| map[i][j]).collect()
            }).collect(),
            edges: vec!["AB", "BC", "CD", "AD"],
            top: 0,
            right: 50
        },
        SubMap {
            map: (0..50).map(|i| {
                (100..150).map(|j| map[i][j]).collect()
            }).collect(),
            edges: vec!["BF", "FG", "CG", "BC"],
            top: 0,
            right: 100
        },
        SubMap {
            map: (50..100).map(|i| {
                (50..100).map(|j| map[i][j]).collect()
            }).collect(),
            edges: vec!["CD", "CG", "GH", "DH"],
            top: 50,
            right: 50
        },
        SubMap {
            map: (100..150).map(|i| {
                (50..100).map(|j| map[i][j]).collect()
            }).collect(),
            edges: vec!["GH", "FG", "EF", "EH"],
            top: 100,
            right: 50
        },
        SubMap {
            map: (100..150).map(|i| {
                (0..50).map(|j| map[i][j]).collect()
            }).collect(),
            edges: vec!["DH", "EH", "AE", "AD"],
            top: 100,
            right: 0
        },
        SubMap {
            map: (150..200).map(|i| {
                (0..50).map(|j| map[i][j]).collect()
            }).collect(),
            edges: vec!["AE", "EF", "BF", "AB"],
            top: 150,
            right: 0
        },
    ];

    let mut position = (0, 0, 0, 0); // submap, y, x, direction

    println!("{:?}", position);
    let mut i =0;
    for instruction in instructions {
        i += 1;
        match instruction {
            Instruction::Clockwise => position.3 = ((position.3+1) as i32).rem_euclid(4),
            Instruction::CounterClockwise => position.3 = ((position.3-1) as i32).rem_euclid(4),
            Instruction::N(n) => {
                for _ in 0..n {
                    position = move_one_2(position, directions[position.3 as usize], &sub_maps);
                    // println!("position: {:?}",position);

                }
            }
        }
        println!("after {:?}, position: {:?}", instruction, position);
        // if i == 10 {
        //     break;
        // }
    }
    let (x, y) = (sub_maps[position.0].right, sub_maps[position.0].top);

    let r = 1000 * (y + position.1+1) + 4 * (x+position.2+1) + position.3;
    println!("{:?}", r);


}

fn find_other_submap(sub_maps: &Vec<SubMap>, edge : &str, sm_id: usize) -> (usize, usize){
    for (i, sm) in sub_maps.iter().enumerate() {
        if i == sm_id {
            continue;
        }
        for (j,edge2) in sm.edges.iter().enumerate() {
            if *edge2 == edge {
                return (i, j);
            }
        }
    }
    panic!()
}

fn move_one_2(position: (usize, i32, i32, i32), d: (i32,i32), sub_maps: &Vec<SubMap>) -> (usize, i32,i32, i32) {
    let (y, x) = (position.1 + d.0, position.2 + d.1);

    let new_position;

    if y < 0 { // change by top
        let edge = sub_maps[position.0 as usize].edges[0];
        let (sm_id, edge_id) = find_other_submap(&sub_maps, edge, position.0 as usize);

        match edge_id {
            2 => {
                new_position = (sm_id, 49, position.2, 3)
            } // arrive by bottom
            3 => {
                new_position = (sm_id, position.2, 0, 0)
            } // arrive by left
            _ => unreachable!()
        }
    } else if x < 0 { // change by left
        let edge = sub_maps[position.0 as usize].edges[3];
        let (sm_id, edge_id) = find_other_submap(&sub_maps, edge, position.0 as usize);

        match edge_id {
            0 => {
                new_position = (sm_id, 0, position.1, 1)
            } // arrive by top
            1 => {
                new_position = (sm_id, position.1, 49, 2)
            } // arrive by right
            3 => {
                new_position = (sm_id, 49 - position.1, 0, 0)
            } // arrive by left
            _ => unreachable!()
        }
    } else if y > 49 { // change by bottom
        let edge = sub_maps[position.0 as usize].edges[2];
        let (sm_id, edge_id) = find_other_submap(&sub_maps, edge, position.0 as usize);

        match edge_id {
            0 => {
                new_position = (sm_id, 0, position.2, 1)
            } // arrive by top
            1 => {
                new_position = (sm_id, position.2, 49, 2)
            } // arrive by right
            _ => unreachable!()
        }
    } else if x > 49 { // change by right
        let edge = sub_maps[position.0 as usize].edges[1];
        let (sm_id, edge_id) = find_other_submap(&sub_maps, edge, position.0 as usize);

        match edge_id {
            1 => {
                new_position = (sm_id, 49 - position.1, 49, 2)
            } // arrive by right
            2 => {
                new_position = (sm_id, 49, position.1, 3)
            } // arrive by bottom
            3 => {
                new_position = (sm_id, position.1, 0, 0)
            } // arrive by left
            _ => unreachable!()
        }
    } else {
        new_position = (position.0, y, x, position.3);
    }

    // println!("{:?}", sub_maps[new_position.0 as usize].map[new_position.1 as usize][new_position.2 as usize]);
    if sub_maps[new_position.0 as usize].map[new_position.1 as usize][new_position.2 as usize] == MapTile::Wall {
        return position;
    }
    return  new_position;
}

fn main() {
    // part1();
    part2();
}

// 46521 too low
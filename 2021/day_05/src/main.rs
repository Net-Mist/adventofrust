use atoi;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_opt,
    sequence::separated_pair,
    IResult,
};

fn num_parser(l: &[u8]) -> IResult<&[u8], usize> {
    map_opt(digit1, atoi::atoi)(l)
}

fn coord_parser(l: &[u8]) -> IResult<&[u8], (usize, usize)> {
    separated_pair(num_parser, char(','), num_parser)(l)
}

fn line_parser(l: &[u8]) -> IResult<&[u8], ((usize, usize), (usize, usize))> {
    separated_pair(coord_parser, tag(" -> "), coord_parser)(l)
}

pub fn main() {
    let (mut map, mut overlaps) = (vec![0u8; 1000 * 1000], 0);
    include_bytes!("../i2.txt")
        .split(|x| *x == b'\n')
        .map(|entry| {
            let ((x, y), (xx, yy)) = line_parser(entry).unwrap().1;
            (x.min(xx), y.min(yy), x.max(xx), y.max(yy))
        })
        .for_each(|(x, y, xx, yy)| {
            let mut mark = |x, y| {
                if map[(x + y * 1000) as usize] == 1 {
                    overlaps += 1;
                }
                map[(x + y * 1000) as usize] += 1;
            };
            if x == xx {
                (y..=yy).for_each(|y| mark(x, y));
            } else if y == yy {
                (x..=xx).for_each(|x| mark(x, y));
            }
        });

    println!("{}", overlaps);
}

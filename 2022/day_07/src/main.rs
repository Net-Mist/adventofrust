use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, sequence::tuple,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum TermLine {
    MoveIn(String),
    MoveBack,
    List,
    ListResultFile((String, usize)),
    ListResultDir(String),
}

#[derive(Debug)]
struct Dir {
    name: String,
    direct_size: usize,
    total_size: usize,
    sub_dir: HashMap<String, usize>,
    parent: Option<usize>,
}

/// Region-based memory management
struct Filesystem {
    dirs: Vec<Dir>,
    current_location: usize,
}

impl Filesystem {
    fn update_parent(self: &mut Self, s: usize, id: usize) {
        self.dirs[id].total_size += s;
        match self.dirs[id].parent {
            Some(id) => self.update_parent(s, id),
            None => (),
        }
    }

    fn new() -> Self{
        Filesystem {
            dirs: vec![
                Dir {
                    name: "/".to_string(),
                    sub_dir: HashMap::new(),
                    direct_size: 0,
                    parent: None,
                    total_size: 0,
                }
            ],
            current_location: 0
        }
    }

    fn move_in(self: &mut Self, name: String) {
        self.current_location = *self.dirs[self.current_location].sub_dir.get(&name).unwrap();

    }

    fn move_back(self: &mut Self) {
        self.current_location = self.dirs[self.current_location].parent.unwrap();
    }

    fn create_folder(self: &mut Self, name: String) {
        let d = Dir {
            name: name.clone(),
            sub_dir: HashMap::new(),
            direct_size: 0,
            parent: Some(self.current_location),
            total_size: 0,
        };
        self.dirs.push(d);
        let l = self.dirs.len() - 1;
        self.dirs[self.current_location].sub_dir.insert(name, l);
    }
}

fn num_parser(l: &str) -> IResult<&str, usize> {
    map_res(digit1, |c: &str| c.parse::<usize>())(l)
}

fn word_parser(l: &str) -> IResult<&str, String> {
    map(take_till(|c| c == ' '), |c: &str| c.to_string())(l)
}

fn ls_dir_result(l: &str) -> IResult<&str, TermLine> {
    map(preceded(tag("dir "), word_parser), |w: String| {
        TermLine::ListResultDir(w)
    })(l)
}

fn ls_file_result(l: &str) -> IResult<&str, TermLine> {
    map(
        tuple((num_parser, word_parser)),
        |w: (usize, String)| TermLine::ListResultFile((w.1, w.0)),
    )(l)
}

fn cd_back(l: &str) -> IResult<&str, TermLine> {
    map(tag("$ cd .."), |_| TermLine::MoveBack)(l)
}

fn cd_in(l: &str) -> IResult<&str, TermLine> {
    map(
        preceded(tag("$ cd "), word_parser),
        |w| TermLine::MoveIn(w),
    )(l)
}

fn list(l: &str) -> IResult<&str, TermLine> {
    map(tag("$ ls"), |_| TermLine::List)(l)
}

fn line_parser(l: &str) -> IResult<&str, TermLine> {
    alt((ls_dir_result, ls_file_result, cd_back, cd_in, list))(l)
}

fn data() -> Filesystem {
    let mut fs = Filesystem::new();
    
    let _a: Vec<_> = include_str!("../i.txt")
        .lines()
        .map(|b| line_parser(b).unwrap().1)
        .map(|l| {
            match l {
                TermLine::MoveIn(n) => {
                    if n == *"/" {
                        fs.current_location = 0;
                    } else {
                        fs.move_in(n);
                    }
                }
                TermLine::MoveBack => (fs.move_back()),
                TermLine::List => (),
                TermLine::ListResultFile((_, size)) => {
                    fs.dirs[fs.current_location].direct_size += size;
                    fs.update_parent(size, fs.current_location);
                }
                TermLine::ListResultDir(name) => {
                    fs.create_folder(name);
                }
            }
        })
        .collect();
    fs
}


fn part1() {
    let d = data();

    let a: usize = d.dirs
        .iter()
        .filter(|d| d.total_size <= 100000)
        .map(|d| d.total_size)
        .sum();
    println!("{a}");

}

fn part2() {
    let d = data();

    let space_av = 70000000 - d.dirs[0].total_size;
    let space_needed = 30000000 - space_av;

    let a = d.dirs
        .iter()
        .map(|d| d.total_size)
        .filter(|s| *s >= space_needed)
        .min();
    println!("{}", a.unwrap())
}

fn main() {
    part1();
    part2();
}

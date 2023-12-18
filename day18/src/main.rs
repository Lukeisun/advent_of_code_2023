use core::panic;
use std::{
    collections::VecDeque,
    env,
    io::{self, Read},
    process::exit,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    UP = 0,    // (-1 ,0)
    DOWN = 1,  // (1,0)
    LEFT = 2,  // (0, -1)
    RIGHT = 3, // (0, 1)
}
impl Dir {
    fn get_dir(&self) -> (i32, i32) {
        match &self {
            Dir::UP => (-1, 0),
            Dir::DOWN => (1, 0),
            Dir::LEFT => (0, -1),
            Dir::RIGHT => (0, 1),
        }
    }
}
fn part2(input: &str) -> String {
    todo!();
}
fn part1(input: &str) -> String {
    let mut dirs = Vec::new();

    for line in lines(input) {
        let line: Vec<&str> = line.split_whitespace().collect();
        let d = line[0];
        let dir = match d {
            "R" => Dir::RIGHT,
            "L" => Dir::LEFT,
            "U" => Dir::UP,
            "D" => Dir::DOWN,
            _ => panic!(),
        };
        let amt = line[1].parse::<u32>().unwrap();
        dirs.push((dir, amt));
    }
    let col_len = dirs
        .iter()
        .filter_map(|x| match x.0 {
            Dir::RIGHT | Dir::LEFT => Some(x.1),
            _ => None,
        })
        .max()
        .unwrap();
    let mut board: VecDeque<VecDeque<char>> = VecDeque::new();
    board.push_front(VecDeque::from(vec!['.'; col_len as usize + 1]));
    let mut cur_idx = (0, 0);
    let mut h_count = 0;
    for d in dirs.iter() {
        let dir = &d.0;
        let dir_t = dir.get_dir();
        let amt = d.1;
        println!("{:?} {:?}", dir, cur_idx);
        match dir {
            Dir::RIGHT => {
                for i in 0..amt {
                    board[cur_idx.0 as usize][cur_idx.1 as usize] = '#';
                    h_count += 1;
                    cur_idx = (cur_idx.0 as i32 + dir_t.0, cur_idx.1 as i32 + dir_t.1);
                    if cur_idx.1 >= board[cur_idx.0 as usize].len() as i32 {
                        for bo in board.iter_mut() {
                            bo.push_back('.');
                        }
                    }
                }
            }
            Dir::LEFT => {
                for i in 0..amt {
                    board[cur_idx.0 as usize][cur_idx.1 as usize] = '#';
                    h_count += 1;
                    cur_idx = (cur_idx.0 as i32 + dir_t.0, cur_idx.1 as i32 + dir_t.1);
                    if cur_idx.1 < 0 {
                        cur_idx = (cur_idx.0, 0);
                        for bo in board.iter_mut() {
                            bo.push_front('.');
                        }
                    }
                }
            }
            Dir::DOWN => {
                for i in 0..amt {
                    // println!("about {:?} {:?}", dir, cur_idx);
                    // println!("{} {}", board.len(), board[0].len());
                    board[cur_idx.0 as usize][cur_idx.1 as usize] = '#';
                    h_count += 1;
                    cur_idx = (cur_idx.0 as i32 + dir_t.0, cur_idx.1 as i32 + dir_t.1);
                    // println!("e {:?} {:?}", dir, cur_idx);
                    if cur_idx.0 >= board.len() as i32 {
                        board.push_back(VecDeque::from(vec!['.'; board[0].len()]));
                    }
                }
            }
            Dir::UP => {
                for i in 0..amt {
                    board[cur_idx.0 as usize][cur_idx.1 as usize] = '#';
                    h_count += 1;
                    cur_idx = (cur_idx.0 as i32 + dir_t.0, cur_idx.1 as i32 + dir_t.1);
                    if cur_idx.0 < 0 {
                        cur_idx = (0, cur_idx.1);
                        board.push_front(VecDeque::from(vec!['.'; board[0].len()]));
                    }
                }
            }
        }
    }
    for r in &board {
        println!("{:?}", r.iter().collect::<String>());
    }
    let mut b = board.clone();
    let num_dots = board
        .iter()
        .map(|x| x.iter().filter(|c| **c == '.').count())
        .fold(0, |acc, n| acc + n);
    // for (i, row) in board.iter().enumerate() {
    //     let mut intersections = 0;
    //     let mut num_in_wall = 0;
    //     let mut inside = false;
    //     for (j, col) in row.iter().enumerate() {
    //         match *col {
    //             '#' => {
    //                 // if num_in_wall == 0 {
    //                 //     intersections += 1;
    //                 // }
    //                 // seen_dot = false;
    //                 num_in_wall += 1;
    //             }
    //             '.' => {
    //                 // num_wall == 0, do nothing
    //                 // num_wall = 1 do nothing
    //                 // if in_wall && !inside {
    //                 //     intersections += 1;
    //                 // }
    //                 // if !seen_dot {
    //                 //     intersections += 1;
    //                 // }
    //                 if num_in_wall == 1 {
    //                     intersections += 1;
    //                 }
    //                 if num_in_wall > 1 {
    //                     if inside {
    //                         intersections += 1;
    //                     } else {
    //                         intersections += 2;
    //                     }
    //                 }
    //                 if intersections % 2 == 1 {
    //                     inside = true;
    //                     b[i][j] = 'I';
    //                 } else {
    //                     inside = false;
    //                 }
    //                 // seen_dot = true;
    //                 h_count += intersections % 2;
    //                 num_in_wall = 0;
    //             }
    //             _ => panic!(),
    //         }
    //         // if *col == '#' && h_neighbors == 0 {
    //         //     intersections += 1;
    //         //     h_neighbors = 1;
    //         // } else if c
    //         // } else {
    //         //     if intersections % 2 == 1 {
    //         //         b[i][j] = 'x';
    //         //     }
    //         // }
    //     }
    // }
    for r in &b {
        println!("{:?}", r.iter().collect::<String>());
    }
    h_count.to_string()
}
fn lines(input: &str) -> Vec<&str> {
    input.split("\n").filter(|x| !x.is_empty()).collect()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("First argument must be the part number");
        exit(1);
    }
    let part = args[1].parse::<u32>().expect("Not a valid number :/");
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    let start = std::time::Instant::now();
    match part {
        1 => {
            let output = part1(&input);
            println!("{output}");
        }
        2 => {
            let output = part2(&input);
            println!("{output}");
        }
        _ => {
            println!("Part number must be 1 or 2");
        }
    }
    eprintln!("{:?}", start.elapsed());
}

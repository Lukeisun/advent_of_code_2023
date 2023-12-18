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
    let mut dirs = Vec::new();
    for line in lines(input) {
        let line: Vec<&str> = line.split_whitespace().collect();
        let hex_s = &line[2][2..7];
        let amt = i64::from_str_radix(hex_s, 16).unwrap();
        let dir = &line[2][7..8];
        // let d = line[0];
        let dir = match dir {
            "0" => Dir::RIGHT,
            "1" => Dir::DOWN,
            "2" => Dir::LEFT,
            "3" => Dir::UP,
            _ => panic!(),
        };
        // let amt = line[1].parse::<u32>().unwrap();
        dirs.push((dir, amt));
    }
    let mut curr_idx = (0, 0);
    let mut vertices = Vec::new();
    vertices.push(curr_idx);
    let mut perim = 0;
    for d in &dirs {
        let dir = &d.0;
        let amt = d.1;
        perim += amt;
        // + 1 in dir at end
        match dir {
            Dir::RIGHT => {
                curr_idx = (curr_idx.0, curr_idx.1 + amt);
                vertices.push(curr_idx);
            }
            Dir::LEFT => {
                curr_idx = (curr_idx.0, curr_idx.1 - amt);
                vertices.push(curr_idx);
            }
            Dir::DOWN => {
                curr_idx = (curr_idx.0 + amt, curr_idx.1);
                vertices.push(curr_idx);
            }
            Dir::UP => {
                curr_idx = (curr_idx.0 - amt, curr_idx.1);
                vertices.push(curr_idx);
            }
        }
    }
    // look into shoelace + picks
    let mut area = 0;
    // neat
    let mut prev = vertices.last().unwrap();
    for curr in vertices.iter().take(vertices.len() - 1) {
        area += (prev.0 * curr.1) - (prev.1 * curr.0);
        prev = curr;
    }

    // for w in vertices.windows(2) {
    //     println!("{:?}", w);
    //     let prev = w[0];
    //     let curr = w[1];
    //     area += (curr.0 + prev.0) * (curr.1 - prev.1);
    // }
    ((area.abs() + perim) / 2 + 1).to_string()
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
    for d in &dirs {
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
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut q = VecDeque::new();
    let r = board.len() as i32;
    let c = board[0].len() as i32;
    // cheating a little bc input
    let mut idx = (r / 2, c / 2);
    q.push_front(idx);
    board[idx.0 as usize][idx.1 as usize] = ' ';
    while let Some((i, j)) = q.pop_front() {
        for dir in dirs {
            let (dx, dy) = dir;
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0 || y < 0 || x >= r || y >= c {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if board[x][y] == '.' {
                q.push_back((x as i32, y as i32));
                board[x][y] = ' ';
            }
        }
    }
    for r in &board {
        println!("{:?}", r.iter().collect::<String>());
    }
    let num_spaces = board
        .iter()
        .map(|x| x.iter().filter(|c| **c == ' ').count())
        .fold(0, |acc, n| acc + n);
    (num_spaces + h_count).to_string()
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

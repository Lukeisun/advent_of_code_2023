use std::{
    collections::VecDeque,
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    todo!();
}
fn part1(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().collect());
    }
    let mut start = (0, 0);
    for (i, r) in board.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }
    let mut q: VecDeque<((i32, i32), usize)> = VecDeque::new();
    q.push_front((start, 0));
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(curr) = q.pop_front() {
        if (curr.1) > 64 || board[curr.0 .0 as usize][curr.0 .1 as usize] == 'O' {
            continue;
        }
        board[curr.0 .0 as usize][curr.0 .1 as usize] = 'O';
        for dir in dirs {
            let (dx, dy) = dir;
            let (x, y) = (curr.0 .0 + dx, curr.0 .1 + dy);
            if x < 0 || y < 0 || x >= board.len() as i32 || y >= board[0].len() as i32 {
                continue;
            }
            if ['O', '#'].contains(&board[x as usize][y as usize]) {
                continue;
            }
            q.push_back(((x, y), curr.1 + 1));
        }
    }
    for b in &board {
        println!("{}", b.iter().collect::<String>());
    }
    board
        .iter()
        .map(|row| row.iter().filter(|col| **col == 'O').count())
        .fold(0, |acc, i| acc + i)
        .to_string()
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

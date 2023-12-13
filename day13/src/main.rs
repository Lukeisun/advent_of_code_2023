use std::{
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    todo!();
}
// 1824 too low
fn part1(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in lines(input) {
        if line.is_empty() {
            for x in &board {
                println!("{}", x.iter().collect::<String>());
            }
            eprintln!("vertical");
            let vertical = find_vertical_reflection(&board);
            sum += vertical;
            eprintln!("{vertical}");
            eprintln!("horiz");
            let horizontal = find_horizontal_reflection(&board);
            eprintln!("{horizontal}");
            sum += horizontal * 100;
            if vertical == 0 && horizontal == 0 {
                panic!("both are 0");
            }
            if vertical > 0 && horizontal > 0 {
                panic!("what do both are > 0");
            }
            board.clear();
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        board.push(chars);
    }
    sum.to_string()
}
fn find_vertical_reflection(board: &Vec<Vec<char>>) -> usize {
    let col_len = board[0].len();
    if col_len % 2 == 0 {
        panic!("no even reflections");
    }
    // let l_split = (col_len / 2, col_len / 2 + 1);
    let split = col_len / 2;
    let mut left_matches = false;
    for j in 0..col_len - 2 {
        'outer: for line in board {
            let line: Vec<char> = line.clone().into_iter().take(col_len - 1 - j).collect();
            for i in 0..line.len() / 2 {
                if line[i] != line[line.len() - 1 - i] {
                    left_matches = false;
                    break 'outer;
                }
            }
            left_matches = true;
        }
        if left_matches == true {
            return split + j;
        }
    }
    // let r_split = (col_len / 2 + 1, col_len / 2 + 2);
    let split = col_len / 2 + 1;
    let mut right_matches = false;
    for j in 0..col_len - 2 {
        'outer: for line in board {
            let line: Vec<char> = line.clone().into_iter().skip(1 + j).collect();
            for i in 0..line.len() / 2 {
                if line[i] != line[line.len() - 1 - i] {
                    right_matches = false;
                    break 'outer;
                }
            }
            right_matches = true;
        }
        if right_matches == true {
            eprintln!("r_split {:?} {}", split, j);
            return split + j;
        }
    }
    0
}
fn find_horizontal_reflection(board: &Vec<Vec<char>>) -> usize {
    let row_len = board.len();
    if row_len % 2 == 0 {
        panic!("no even reflections");
    }
    let col_len = board[0].len();
    let mut iters: Vec<_> = board.into_iter().map(|n| n.into_iter()).collect();
    let transposed_board: Vec<Vec<char>> = (0..col_len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect();
    let split = row_len / 2;
    let mut up_matches = false;
    for j in 0..row_len - 2 {
        'outer: for line in &transposed_board {
            let line: Vec<char> = line.clone().into_iter().take(row_len - 1 - j).collect();
            for i in 0..line.len() / 2 {
                if line[i] != line[line.len() - 1 - i] {
                    up_matches = false;
                    break 'outer;
                }
            }
            up_matches = true;
        }
        if up_matches == true {
            return split + j;
        }
    }
    let split = row_len / 2 + 1;
    let mut down_matches = false;
    for j in 0..row_len - 2 {
        'outer: for line in &transposed_board {
            let line: Vec<char> = line.clone().into_iter().skip(1 + j).collect();
            for i in 0..line.len() / 2 {
                if line[i] != line[line.len() - 1 - i] {
                    down_matches = false;
                    break 'outer;
                }
            }
            down_matches = true;
        }
        if down_matches == true {
            return split + j;
        }
    }
    0
}
fn lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
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
    dbg!(start.elapsed());
}

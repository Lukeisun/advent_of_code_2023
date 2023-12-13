use std::{
    env,
    io::{self, Read},
    iter,
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
    // left
    let split = col_len / 2;
    for i in (1..split + 1).rev() {
        let mut is_mirror = true;
        for row in board {
            println!("{:?}", row);
            let l_split = &row[0..0 + i];
            let rev: &Vec<char> = &row.clone().into_iter().rev().collect();
            let r_split = &rev[0..0 + i];
            println!(
                "{:?} {:?}",
                l_split,
                *r_split.clone().into_iter().rev().collect()
            );
            if l_split != r_split {
                is_mirror = false;
                break;
            }
        }
        if is_mirror {
            return i;
        }
    }
    for i in (1..split + 1).rev() {
        let mut is_mirror = true;
        for row in board {
            // col_len -2 * i
            let shift = col_len - 2 * i;
            println!("{:?}", row);
            let l_split = &row[shift..shift + i];
            let rev: &Vec<char> = &row.clone().into_iter().rev().collect();
            let r_split = &rev[shift..shift + i];
            println!("{:?} {:?}", l_split, r_split);
            if l_split != r_split {
                is_mirror = false;
                break;
            }
        }
        if is_mirror {
            return (col_len - 2 * i) + i;
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
    find_vertical_reflection(&transposed_board)
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

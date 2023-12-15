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
            eprintln!("res {vertical}");
            // eprintln!("horiz");
            let horizontal = find_horizontal_reflection(&board);
            // eprintln!("{horizontal}");
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
    // left edge
    // cols [0, col_len -1)
    println!("COL_LEN {col_len}");
    'outer: for len in (1..col_len - 1).rev() {
        for row in board {
            println!("{:?}", row);
            for i in 0..=len / 2 {
                println!("i {i} len {len} idx {}", len - i);
                if row[i] != row[len - i] {
                    continue 'outer;
                }
            }
        }
        eprintln!("LEFT MATCHES");
        // there is a match
        return len / 2;
    }
    // right edge
    // (0, col_len-1]
    // 1 : len
    // 2 : len - 1
    // 3 : len - 2...
    'outer: for len in (1..col_len - 1).rev() {
        for row in board {
            println!("{:?}", row);
            for i in 1..=len / 2 {
                println!("i {i} len {len} idx {}", len - i);
                if row[i] != row[len - i] {
                    continue 'outer;
                }
            }
        }
        eprintln!("RIGHT MATCHES");
        // there is a match
        return len / 2;
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

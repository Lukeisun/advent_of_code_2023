use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().collect());
    }
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for i in 1..=1000 {
        // eprintln!("{i}");
        for dir in dirs {
            move_dir(&mut board, dir);
        }
        let mut load = 0;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] != 'O' {
                    continue;
                }
                load += board.len() - i;
            }
        }
        if load == 64 {
            println!("{load} {i}");
        }
    }
    board
        .iter()
        .for_each(|r| eprintln!("{}", r.iter().collect::<String>()));
    println!("");
    let mut load = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] != 'O' {
                continue;
            }
            load += board.len() - i;
        }
    }
    load.to_string()
}
fn part1(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().collect());
    }
    // board
    //     .iter()
    //     .for_each(|r| eprintln!("{}", r.iter().collect::<String>()));
    move_dir(&mut board, (-1, 0));
    board
        .iter()
        .for_each(|r| eprintln!("{}", r.iter().collect::<String>()));
    // check load;
    let mut load = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] != 'O' {
                continue;
            }
            load += board.len() - i;
        }
    }
    load.to_string()
}
fn move_dir(board: &mut Vec<Vec<char>>, dir: (i32, i32)) -> () {
    let row_len = board.len();
    let col_len = board[0].len();
    for i in 0..row_len {
        for j in 0..col_len {
            if board[i][j] != 'O' {
                continue;
            }
            let mut new_r = i as i32 + dir.0;
            let mut new_c = j as i32 + dir.1;
            let mut curr_r = i;
            let mut curr_c = j;
            if new_r < 0 || new_c < 0 || new_r as usize >= row_len || new_c as usize >= col_len {
                continue;
            }
            'outer: while board[new_r as usize][new_c as usize] != '#' {
                while board[new_r as usize][new_c as usize] == 'O' {
                    new_r += dir.0;
                    new_c += dir.1;
                    if new_r < 0
                        || new_c < 0
                        || new_r as usize >= row_len
                        || new_c as usize >= col_len
                        || board[new_r as usize][new_c as usize] == '#'
                    {
                        break 'outer;
                    }
                }
                // board[new_r as usize][new_c as usize] = 'O';
                let temp = board[new_r as usize][new_c as usize];
                board[new_r as usize][new_c as usize] = board[curr_r][curr_c];
                board[curr_r][curr_c] = temp;
                curr_r = new_r as usize;
                curr_c = new_c as usize;
                new_r += dir.0;
                new_c += dir.1;
                if new_r < 0 || new_c < 0 || new_r as usize >= row_len || new_c as usize >= col_len
                {
                    break;
                }
            }
        }
    }
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

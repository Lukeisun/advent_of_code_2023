use std::{
    env,
    io::{self, Read},
    process::exit,
};

// 36156 too low
fn part2(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in lines(input) {
        if line.is_empty() {
            if board.is_empty() {
                break;
            }
            // for x in &board {
            //     println!("{}", x.iter().collect::<String>());
            // }
            let vertical = find_vertical_reflection(&board);
            let horizontal = find_horizontal_reflection(&board);
            let smudge_h = find_smudged_horizontal(&board);
            let smudge_v = find_smudeged_vertical(&board);
            println!("vert {vertical}");
            println!("hor {horizontal}");
            println!("sh {:?}", smudge_h);
            println!("sv {:?}", smudge_v);
            if vertical != 0 {
                let v_val: Vec<&usize> = smudge_v.iter().filter(|x| **x != vertical).collect();
                if !v_val.is_empty() {
                    sum += v_val[0];
                }
                if !smudge_h.is_empty() {
                    sum += smudge_h[0] * 100;
                }
            }
            if horizontal != 0 {
                let h_val: Vec<&usize> = smudge_h.iter().filter(|x| **x != horizontal).collect();
                if !h_val.is_empty() {
                    sum += h_val[0] * 100;
                }
                if h_val.is_empty() && smudge_v.is_empty() {
                    panic!("empty");
                }
                if !smudge_v.is_empty() {
                    sum += smudge_v[0];
                }
            }
            if vertical == 0 && horizontal == 0 {
                panic!("both are 0");
            }
            if vertical > 0 && horizontal > 0 {
                panic!("what do both are > 0");
            }
            println!("{sum}");
            board.clear();
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        board.push(chars);
    }
    sum.to_string()
}
fn part1(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in lines(input) {
        if line.is_empty() {
            if board.is_empty() {
                break;
            }
            for x in &board {
                println!("{}", x.iter().collect::<String>());
            }
            let vertical = find_vertical_reflection(&board);
            sum += vertical;
            let horizontal = find_horizontal_reflection(&board);
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
fn find_smudged_horizontal(board: &Vec<Vec<char>>) -> Vec<usize> {
    let prev_row = &board[0];
    let mut prev_rows = Vec::new();
    prev_rows.push(prev_row);
    let mut res = Vec::new();
    'outer: for r in 1..board.len() {
        // println!("{:?}", prev_rows);
        let mut differences = 0;
        for i in r..board.len() {
            // println!("i {} r {}, prev_rows {:?}", i, r, prev_rows.len());
            let idx = (prev_rows.len() - 1).checked_sub(i - r);
            if idx.is_none() {
                break;
            }
            let idx = idx.unwrap();
            let diff = board[i]
                .iter()
                .zip(prev_rows[idx])
                .into_iter()
                .filter(|a| a.0 != a.1)
                .count();
            differences += diff;
            if !&board[i].eq(prev_rows[idx]) && differences > 1 {
                prev_rows.push(&board[r]);
                continue 'outer;
            }
        }
        res.push(r);
    }
    res
}
fn find_horizontal_reflection(board: &Vec<Vec<char>>) -> usize {
    let prev_row = &board[0];
    let mut prev_rows = Vec::new();
    prev_rows.push(prev_row);
    'outer: for r in 1..board.len() {
        // println!("{:?}", prev_rows);
        for i in r..board.len() {
            let idx = (prev_rows.len() - 1).checked_sub(i - r);
            if idx.is_none() {
                return r;
            }
            let idx = idx.unwrap();
            if !&board[i].eq(prev_rows[idx]) {
                prev_rows.push(&board[r]);
                continue 'outer;
            }
        }
        return r;
    }
    0
}
fn find_smudeged_vertical(board: &Vec<Vec<char>>) -> Vec<usize> {
    let mut iters: Vec<_> = board.into_iter().map(|n| n.into_iter()).collect();
    let transposed_board: Vec<Vec<char>> = (0..board[0].len())
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect();
    find_smudged_horizontal(&transposed_board)
}
fn find_vertical_reflection(board: &Vec<Vec<char>>) -> usize {
    let mut iters: Vec<_> = board.into_iter().map(|n| n.into_iter()).collect();
    let transposed_board: Vec<Vec<char>> = (0..board[0].len())
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect();
    find_horizontal_reflection(&transposed_board)
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

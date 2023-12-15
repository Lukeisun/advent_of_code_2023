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
        println!("l {line}");
        if line.is_empty() {
            if board.is_empty() {
                break;
            }
            for x in &board {
                println!("{}", x.iter().collect::<String>());
            }
            eprintln!("vertical");
            let vertical = find_vertical_reflection(&board);
            sum += vertical;
            eprintln!("v res {vertical}");
            eprintln!("horiz");
            let horizontal = find_horizontal_reflection(&board);
            eprintln!("h res {horizontal}");
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
fn find_horizontal_reflection(board: &Vec<Vec<char>>) -> usize {
    // get above and below a split
    for x in board {
        println!("{}", x.iter().collect::<String>());
    }
    let prev_row = &board[0];
    let mut prev_rows = Vec::new();
    prev_rows.push(prev_row);
    'outer: for r in 1..board.len() {
        // println!("{:?}", prev_rows);
        for i in r..board.len() {
            println!("i {} r {}, prev_rows {:?}", i, r, prev_rows.len());
            let idx = (prev_rows.len() - 1).checked_sub(i - r);
            if idx.is_none() {
                return r;
            }
            let idx = idx.unwrap();
            println!("idx {}", idx);
            if !&board[i].eq(prev_rows[idx]) {
                prev_rows.push(&board[r]);
                continue 'outer;
            }
            println!("{:?} == {:?}", &board[i], prev_rows[idx]);
        }
        return r;
    }
    0
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
// fn is_symmetric(s: &[char]) -> bool {
//     for i in 0..s.len() / 2 {
//         if s[i] != s[s.len() - i - 1] {
//             return false;
//         }
//     }
//     true
// }
// fn find_vertical_reflection(board: &Vec<Vec<char>>) -> usize {
//     let col_len = board[0].len();
//     // left edge
//     // cols [0, col_len -1)
//     println!("COL_LEN {col_len}");
//     'outer: for si in 0..col_len - 2 {
//         for row in board {
//             let slice = &row[0..col_len - 1 - si];
//             println!("{:?}", slice);
//             if !is_symmetric(slice) {
//                 continue 'outer;
//             }
//         }
//         println!("SI {si}");
//         println!("NEW SI {}", (col_len - 1 - si) / 2);
//         return (col_len - 1 - si) / 2;
//     }
//     eprintln!("NO LEFT");
//     // right edge
//     // (0, col_len-1]
//     // 1 : len
//     // 2 : len - 1
//     // 3 : len - 2...
//     'outer: for si in 1..col_len - 1 {
//         for row in board {
//             let slice = &row[si..col_len];
//             println!("{:?}", slice);
//             if !is_symmetric(slice) {
//                 continue 'outer;
//             }
//         }
//         println!("NEW SI {si}");
//         println!("NEW SI {}", (si + col_len) / 2);
//         return (si + col_len) / 2;
//     }
//     eprintln!("NO RIGHT");
//     0
// }
// fn find_horizontal_reflection(board: &Vec<Vec<char>>) -> usize {
//     let row_len = board.len();
//     if row_len % 2 == 0 {
//         panic!("no even reflections");
//     }
//     let col_len = board[0].len();
//     let mut iters: Vec<_> = board.into_iter().map(|n| n.into_iter()).collect();
//     let transposed_board: Vec<Vec<char>> = (0..col_len)
//         .map(|_| {
//             iters
//                 .iter_mut()
//                 .map(|n| *n.next().unwrap())
//                 .collect::<Vec<char>>()
//         })
//         .collect();
//     println!("do the horizo");
//     find_vertical_reflection(&transposed_board)
// }
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

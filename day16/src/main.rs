use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};

#[derive(Debug, Clone, PartialEq)]
enum Dirs {
    UP,    // (-1 ,0)
    DOWN,  // (1,0)
    LEFT,  // (0, -1)
    RIGHT, // (0, 1)
}
fn part2(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().collect());
    }
    let row_len = board.len();
    let col_len = board[0].len();
    let mut max = 0;
    // DOWN
    for j in 0..col_len {
        let mut visited = HashMap::new();
        go(&board, &Dirs::DOWN, (0, j), &mut visited);
        max = max.max(visited.len());
    }
    // UP
    for j in 0..col_len {
        let mut visited = HashMap::new();
        go(&board, &Dirs::UP, (row_len - 1, j), &mut visited);
        max = max.max(visited.len());
    }
    // LEFT
    for i in 0..row_len {
        let mut visited = HashMap::new();
        go(&board, &Dirs::LEFT, (i, col_len - 1), &mut visited);
        max = max.max(visited.len());
    }
    // RIGHT
    for i in 0..row_len {
        let mut visited = HashMap::new();
        go(&board, &Dirs::RIGHT, (i, 0), &mut visited);
        max = max.max(visited.len());
    }
    println!("{max}");
    max.to_string()
}
fn part1(input: &str) -> String {
    let mut board: Vec<Vec<char>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().collect());
    }
    let mut visited = HashMap::new();
    go(&board, &Dirs::RIGHT, (0, 0), &mut visited);
    // for (i, row) in board.iter_mut().enumerate() {
    //     for (j, col) in row.iter_mut().enumerate() {
    //         if *col == '\\' {
    //             *col = ']';
    //         }
    //     }
    // }
    // println!("{:?}", visited);
    // for (i, row) in board.iter_mut().enumerate() {
    //     for (j, col) in row.iter_mut().enumerate() {
    //         if *col == '\\' {
    //             *col = ']';
    //         }
    //         if visited.contains_key(&(i, j)) {
    //             *col = '#';
    //         }
    //     }
    // }
    // board
    //     .iter()
    //     .for_each(|x| println!("{:?}", x.iter().collect::<String>()));
    // let num_h = board
    //     .iter()
    //     .map(|x| x.iter().filter(|y| **y == '#').count())
    //     .fold(0, |a, b| a + b);
    // println!("{:?} {num_h}", visited.len());
    visited.len().to_string()
    // num_h.to_string()
}
fn go(
    board: &Vec<Vec<char>>,
    dir_enum: &Dirs,
    idx: (usize, usize),
    visited: &mut HashMap<(usize, usize), Vec<Dirs>>,
) {
    let dir = get_dir(&dir_enum);
    let mut curr_r = idx.0 as i32;
    let mut curr_c = idx.1 as i32;
    let mut current_char = board[idx.0][idx.1];
    let row_len = board.len() as i32;
    let col_len = board[0].len() as i32;
    let specials = ['/', '\\', '|', '-'];
    while !specials.contains(&current_char) {
        // board[curr_r as usize][curr_c as usize] = '#';
        visited.insert((curr_r as usize, curr_c as usize), vec![]);
        curr_r += dir.0;
        curr_c += dir.1;
        if curr_r < 0 || curr_c < 0 || curr_r >= row_len || curr_c >= col_len {
            return;
        }
        current_char = board[curr_r as usize][curr_c as usize];
    }
    // board.iter().for_each(|x| println!("{:?}", x));
    // println!("");
    let vec = visited
        .entry((curr_r as usize, curr_c as usize))
        .or_insert(vec![]);
    if current_char == '/' {
        let dir_enum = match dir_enum {
            Dirs::RIGHT => Dirs::UP,
            Dirs::UP => Dirs::RIGHT,
            Dirs::LEFT => Dirs::DOWN,
            Dirs::DOWN => Dirs::LEFT,
        };
        if vec.contains(&dir_enum) {
            return;
        }
        vec.push(dir_enum.clone());
        let dir = get_dir(&dir_enum);
        let new_r = curr_r + dir.0;
        let new_c = curr_c + dir.1;
        if in_range(new_r, new_c, row_len, col_len) {
            go(board, &dir_enum, (new_r as usize, new_c as usize), visited);
        }
    } else if current_char == '\\' {
        let dir_enum = match dir_enum {
            Dirs::RIGHT => Dirs::DOWN,
            Dirs::DOWN => Dirs::RIGHT,
            Dirs::LEFT => Dirs::UP,
            Dirs::UP => Dirs::LEFT,
        };
        if vec.contains(&dir_enum) {
            return;
        }
        vec.push(dir_enum.clone());
        let dir = get_dir(&dir_enum);
        let new_r = curr_r + dir.0;
        let new_c = curr_c + dir.1;
        if in_range(new_r, new_c, row_len, col_len) {
            go(board, &dir_enum, (new_r as usize, new_c as usize), visited);
        }
    } else if current_char == '|' {
        let up = get_dir(&Dirs::UP);
        let down = get_dir(&Dirs::DOWN);
        let up_row = curr_r + up.0;
        let up_col = curr_c + up.1;
        if in_range(up_row, up_col, row_len, col_len) && !matches!(dir_enum, Dirs::DOWN) {
            go(
                board,
                &Dirs::UP,
                (up_row as usize, up_col as usize),
                visited,
            );
        }
        let down_row = curr_r + down.0;
        let down_col = curr_c + down.1;
        if in_range(down_row, down_col, row_len, col_len) && !matches!(dir_enum, Dirs::UP) {
            go(
                board,
                &Dirs::DOWN,
                (down_row as usize, down_col as usize),
                visited,
            );
        }
    } else if current_char == '-' {
        let left = get_dir(&Dirs::LEFT);
        let right = get_dir(&Dirs::RIGHT);
        let left_row = curr_r + left.0;
        let left_col = curr_c + left.1;
        if in_range(left_row, left_col, row_len, col_len) && !matches!(dir_enum, Dirs::RIGHT) {
            go(
                board,
                &Dirs::LEFT,
                (left_row as usize, left_col as usize),
                visited,
            );
        }
        let right_row = curr_r + right.0;
        let right_col = curr_c + right.1;
        if in_range(right_row, right_col, row_len, col_len) && !matches!(dir_enum, Dirs::LEFT) {
            go(
                board,
                &Dirs::RIGHT,
                (right_row as usize, right_col as usize),
                visited,
            );
        }
    }
}
fn in_range(row: i32, col: i32, row_len: i32, col_len: i32) -> bool {
    return 0 <= row && row < row_len && 0 <= col && col < col_len;
}
fn get_dir(dir: &Dirs) -> (i32, i32) {
    match dir {
        Dirs::UP => (-1, 0),
        Dirs::DOWN => (1, 0),
        Dirs::LEFT => (0, -1),
        Dirs::RIGHT => (0, 1),
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

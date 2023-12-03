use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};
#[derive(Debug)]
enum Token {
    Digit(u8),
    Symbol,
    Period,
    Gear,
}

fn part2(input: &str) -> String {
    let mut schematic: Vec<Vec<Token>> = Vec::new();
    let mut sum = 0;
    for line in lines(input) {
        let mut row: Vec<Token> = line
            .chars()
            .into_iter()
            .map(|x| match x.to_digit(10) {
                Some(symbol) => Token::Digit(symbol as u8),
                _ => match x {
                    '.' => Token::Period,
                    '*' => Token::Gear,
                    _ => Token::Symbol,
                },
            })
            .collect();
        row.push(Token::Period);
        schematic.push(row);
    }
    let mut gears_map = HashMap::new();
    let mut map = HashMap::new();
    let schematic = &*schematic;
    // get digits and map to idxs
    for (i, row) in schematic.into_iter().enumerate() {
        let mut temp: String = String::new();
        let mut indexs: Vec<(usize, usize)> = Vec::new();
        let mut starting_index: (usize, usize) = (0, 0);
        for (j, col) in row.into_iter().enumerate() {
            match col {
                Token::Digit(x) => {
                    if temp.is_empty() {
                        starting_index = (i, j);
                    }
                    indexs.push((i, j));
                    temp.push(char::from_digit(*x as u32, 10).unwrap());
                }
                _ => {
                    if let Ok(value) = temp.parse::<u32>() {
                        for index in indexs.clone().into_iter() {
                            map.insert(index, (value, starting_index));
                        }
                        indexs = Vec::new();
                        temp.clear();
                    }
                    if let Token::Gear = col {
                        gears_map.insert((i, j), ());
                    }
                }
            }
        }
    }
    let dir_array: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
    ];
    let row_len = schematic.len();
    let col_len = schematic[0].len();
    for (gear_idx, _) in gears_map {
        //                          (value, (starting index))
        let mut first_match: Option<(u32, (usize, usize))> = None;
        let mut second_match: Option<(u32, (usize, usize))> = None;
        for dir in dir_array {
            let di = dir.0 + gear_idx.0 as i32;
            let dj = dir.1 + gear_idx.1 as i32;
            if dj >= col_len as i32 || di >= row_len as i32 || di < 0 || dj < 0 {
                continue;
            }
            let curr = (di as usize, dj as usize);
            if !map.contains_key(&curr) {
                continue;
            }
            if !first_match.is_some() {
                first_match = map.get(&curr).copied();
            } else if !second_match.is_some() {
                let temp_option = map.get(&curr).copied();
                let temp = temp_option.unwrap();
                let first_match_idx = first_match.unwrap().1;
                let curr_idx = temp.1;
                if curr_idx == first_match_idx {
                    continue;
                }
                second_match = temp_option;
            } else {
                let first = first_match.unwrap();
                let second = second_match.unwrap();
                let third = map.get(&curr).copied().unwrap();
                if first.1 == third.1 || second.1 == third.1 {
                    continue;
                }
                first_match = None;
                second_match = None;
                break;
            }
        }
        if first_match.is_some() && second_match.is_some() {
            sum += first_match.unwrap().0 * second_match.unwrap().0;
        }
    }
    sum.to_string()
}
fn part1(input: &str) -> String {
    let mut schematic: Vec<Vec<Token>> = Vec::new();
    for line in lines(input) {
        let mut row: Vec<Token> = line
            .chars()
            .into_iter()
            .map(|x| match x.to_digit(10) {
                Some(token) => Token::Digit(token as u8),
                _ => match x {
                    '.' => Token::Period,
                    _ => Token::Symbol,
                },
            })
            .collect();
        row.push(Token::Period);
        schematic.push(row);
    }
    let mut count = 0;
    let dir_array: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let row_len = schematic.len();
    let col_len = schematic[0].len();
    let schematic = &*schematic;
    for (i, row) in schematic.into_iter().enumerate() {
        let mut temp: String = String::new();
        let mut is_part: bool = false;
        for (j, col) in row.into_iter().enumerate() {
            match col {
                Token::Digit(x) => {
                    temp.push(char::from_digit(*x as u32, 10).unwrap());
                    for dir in dir_array {
                        let di = i as i32 + dir.0;
                        let dj = j as i32 + dir.1;
                        if di >= row_len as i32 || dj >= col_len as i32 || di < 0 || dj < 0 {
                            continue;
                        }
                        if let Token::Symbol = &schematic[di as usize][dj as usize] {
                            is_part = true;
                        }
                    }
                }
                _ => {
                    if !temp.is_empty() && is_part {
                        count += temp.parse::<u32>().unwrap();
                    }
                    is_part = false;
                    temp.clear();
                }
            }
        }
    }
    count.to_string()
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
}

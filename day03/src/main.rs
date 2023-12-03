use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};
#[derive(Debug)]
enum Symbol {
    Digit(u8),
    Character(char),
    Period(char),
}
#[derive(Debug)]
enum SymbolG {
    Digit(u8),
    Character(char),
    Period(char),
    Gear(char),
}

fn part2(input: &str) -> String {
    let mut schematic: Vec<Vec<SymbolG>> = Vec::new();
    let mut sum = 0;
    for line in lines(input) {
        let row = line
            .chars()
            .into_iter()
            .map(|x| match x.to_digit(10) {
                Some(symbol) => SymbolG::Digit(symbol as u8),
                _ => match x {
                    '.' => SymbolG::Period(x),
                    '*' => SymbolG::Gear(x),
                    _ => SymbolG::Character(x),
                },
            })
            .collect();
        schematic.push(row);
    }
    let mut map = HashMap::new();
    let schematic = &*schematic;
    // get digits and map to idxs
    for (i, row) in schematic.into_iter().enumerate() {
        let mut temp: String = String::new();
        let mut indexs: Vec<(usize, usize)> = Vec::new();
        for (j, col) in row.into_iter().enumerate() {
            match col {
                SymbolG::Digit(x) => {
                    indexs.push((i, j));
                    temp.push(char::from_digit(*x as u32, 10).unwrap());
                }
                _ => {
                    if let Ok(value) = temp.parse::<u32>() {
                        for index in indexs.clone().into_iter() {
                            map.insert(index, value);
                        }
                        indexs = Vec::new();
                        temp.clear();
                    }
                }
            }
        }
        // in case last col
        if let Ok(value) = temp.parse::<u32>() {
            for index in indexs.clone().into_iter() {
                map.insert(index, value);
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
    for (i, row) in schematic.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if let SymbolG::Gear(_) = col {
                let mut first_match: Option<u32> = None;
                let mut first_match_idx = (std::usize::MAX, std::usize::MAX);
                let mut second_match: Option<u32> = None;
                let mut second_match_idx = (std::usize::MAX, std::usize::MAX);
                for direction in dir_array {
                    let dir_i = i as i32 + direction.0;
                    let dir_j = j as i32 + direction.1;
                    if dir_i >= row_len as i32 || dir_j >= col_len as i32 || dir_i < 0 || dir_j < 0
                    {
                        continue;
                    }
                    let t: (usize, usize) = (dir_i as usize, dir_j as usize);
                    if map.contains_key(&t) {
                        if !first_match.is_some() {
                            first_match = map.get(&t).copied();
                            first_match_idx = t;
                        } else if !second_match.is_some() {
                            // println!("{}", first_match.unwrap());
                            let temp = map.get(&t).copied();
                            let f_i = first_match_idx.0;
                            let f_j = first_match_idx.1;
                            if f_i == t.0 {
                                let distance = (f_j as i32 - t.1 as i32).abs();
                                // println!("{}", distance);
                                if distance == 2 {
                                    let middle = f_j.max(t.1) - 1;
                                    // println!("{} {}", f_i, middle);
                                    if !map.contains_key(&(f_i, middle)) {
                                        second_match = temp;
                                        second_match_idx = t;
                                    }
                                }
                                continue;
                            }
                            second_match_idx = t;
                            second_match = temp;
                        } else {
                            let f_i = first_match_idx.0;
                            let f_j = first_match_idx.1;
                            let s_i = second_match_idx.0;
                            let s_j = second_match_idx.1;
                            if f_i == t.0 {
                                let distance = (f_j as i32 - t.1 as i32).abs();
                                if distance == 2 {
                                    let middle = f_j.max(t.1) - 1;
                                    if !map.contains_key(&(f_i, middle)) {
                                        continue;
                                    }
                                } else {
                                    continue;
                                }
                            }
                            if s_i == t.0 {
                                let distance = (s_j as i32 - t.1 as i32).abs();
                                if distance == 2 {
                                    let middle = s_j.max(t.1) - 1;
                                    if !map.contains_key(&(s_i, middle)) {
                                        continue;
                                    }
                                } else {
                                    continue;
                                }
                            }
                            first_match = None;
                            second_match = None;
                            break;
                        }
                    }
                }
                if first_match.is_some() && second_match.is_some() {
                    println!("{} {}", first_match.unwrap(), second_match.unwrap());
                    sum += first_match.unwrap() * second_match.unwrap();
                }
            }
        }
    }
    sum.to_string()
}
fn part1(input: &str) -> String {
    let mut schematic: Vec<Vec<Symbol>> = Vec::new();
    for line in lines(input) {
        let row = line
            .chars()
            .into_iter()
            .map(|x| match x.to_digit(10) {
                Some(symbol) => Symbol::Digit(symbol as u8),
                _ => match x {
                    '.' => Symbol::Period(x),
                    _ => Symbol::Character(x),
                },
            })
            .collect();
        schematic.push(row);
    }
    // println!("{:#?}", schematic);
    let mut count = 0;
    let dir_array: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, -1),
        // these are last because if we move beyond the column len
        // there is no next symbol to read. cant handle if its a part etc
        (1, 1),
        (0, 1),
        (-1, 1),
    ];
    let row_len = schematic.len();
    let col_len = schematic[0].len();
    let schematic = &*schematic;
    for (i, row) in schematic.into_iter().enumerate() {
        let mut temp: String = String::new();
        let mut is_part: bool = false;
        for (j, col) in row.into_iter().enumerate() {
            match col {
                Symbol::Period(_) | Symbol::Character(_) => {
                    if !temp.is_empty() && is_part {
                        count += temp.parse::<u32>().unwrap();
                    }
                    is_part = false;
                    temp.clear();
                }
                Symbol::Digit(x) => {
                    temp.push(char::from_digit(*x as u32, 10).unwrap());
                    for direction in dir_array {
                        let dir_i = i as i32 + direction.0;
                        let dir_j = j as i32 + direction.1;
                        if dir_j >= col_len as i32 {
                            if !temp.is_empty() && is_part {
                                count += temp.parse::<u32>().unwrap();
                            }
                            is_part = false;
                            temp.clear();
                            break;
                        }
                        if dir_i >= row_len as i32 || dir_i < 0 || dir_j < 0 {
                            continue;
                        }
                        let symbol = &schematic[dir_i as usize][dir_j as usize];
                        if let Symbol::Character(_) = symbol {
                            is_part = true;
                        }
                    }
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

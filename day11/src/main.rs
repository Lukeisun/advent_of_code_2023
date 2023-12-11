use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    let mut game: Vec<Vec<String>> = Vec::new();
    let mut rows: Vec<usize> = Vec::new();
    for (i, line) in lines(input).into_iter().enumerate() {
        let line: Vec<String> = line.chars().map(|x| x.to_string()).collect();
        game.push(line.clone());
        if line.iter().all(|x| *x == ".") {
            rows.push(i);
        }
    }
    let row_len = game.len();
    let col_len = game[0].len();
    let mut cols: Vec<usize> = Vec::new();
    let mut galaxies = Vec::new();
    for col in 0..col_len {
        let mut all_dots = true;
        for row in 0..row_len {
            if game[row][col] == "#" {
                all_dots = false;
                galaxies.push((row, col));
            }
        }
        if all_dots {
            cols.push(col);
        }
    }
    let expansion_factor = 999_999;
    let mut sum = 0;
    let mut pairs = HashMap::new();
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            let other = &galaxies[j];
            if galaxy == other {
                continue;
            }
            let greater_row = galaxy.0.max(other.0);
            let lesser_row = galaxy.0.min(other.0);
            let greater_col = galaxy.1.max(other.1);
            let lesser_col = galaxy.1.min(other.1);
            let greater_row_count = rows.iter().filter(|x| *x < &greater_row).count();
            let lesser_row_count = rows.iter().filter(|x| *x < &lesser_row).count();
            let greater_col_count = cols.iter().filter(|x| *x < &greater_col).count();
            let lesser_col_count = cols.iter().filter(|x| *x < &lesser_col).count();
            let x = greater_row + greater_row_count * expansion_factor
                - (lesser_row + lesser_row_count * expansion_factor);
            let y = greater_col + greater_col_count * expansion_factor
                - (lesser_col + lesser_col_count * expansion_factor);
            pairs.insert((galaxy, other), x + y);
            sum += x + y;
        }
    }
    sum.to_string()
}

fn part1(input: &str) -> String {
    let mut game: Vec<Vec<String>> = Vec::new();
    for line in lines(input) {
        let line: Vec<String> = line.chars().map(|x| x.to_string()).collect();
        game.push(line.clone());
        if line.iter().all(|x| *x == ".") {
            game.push(line);
        }
    }
    let row_len = game.len();
    let col_len = game[0].len();
    let mut cols: Vec<usize> = Vec::new();
    for col in 0..col_len {
        let mut all_dots = true;
        for row in 0..row_len {
            if game[row][col] == "#" {
                all_dots = false;
            }
        }
        if all_dots {
            cols.push(col);
        }
    }
    // just for debugging
    let mut g_count = 1;
    for row in &mut game {
        for col in row {
            if col == "#" {
                *col = g_count.to_string();
                g_count += 1
            }
        }
    }
    //
    for (i, col) in cols.iter().enumerate() {
        for row in &mut game {
            row.insert(*col + i, ".".to_string());
        }
    }
    let row_len = game.len();
    let col_len = game[0].len();
    let mut galaxies = Vec::new();
    for row in 0..row_len {
        for col in 0..col_len {
            if game[row][col].parse::<u32>().is_ok() {
                galaxies.push((row, col));
            }
        }
    }

    let mut sum = 0;
    let mut pairs = HashMap::new();
    // println!("g {}", galaxies.len());
    for galaxy in &galaxies {
        for other in &galaxies {
            if galaxy == other {
                continue;
            }
            let x = (galaxy.1 as i32 - other.1 as i32).abs();
            let y = (galaxy.0 as i32 - other.0 as i32).abs();
            if pairs.contains_key(&(other, galaxy)) {
                continue;
            }
            pairs.insert((galaxy, other), x + y);
            sum += x + y;
        }
    }
    // println!(" counter {:?}", pairs);
    // let str_repr: Vec<String> = game
    //     .into_iter()
    //     .map(|x| x.into_iter().map(|c| c.to_string()).collect())
    //     .collect();
    // str_repr.into_iter().for_each(|x| {
    //     println!("{}", x);
    // });
    sum.to_string()
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
    dbg!(start.elapsed());
}

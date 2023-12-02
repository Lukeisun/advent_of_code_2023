use std::{
    env,
    io::{self, Read},
    process::exit,
};
fn part2(input: &str) -> String {
    let mut possible_game_ids: Vec<u32> = Vec::new();
    for line in input.split("\n").filter(|x| !x.is_empty()) {
        let line = line.to_string();
        let game_split: Vec<&str> = line.split(":").collect();
        let sets: Vec<&str> = game_split[1].split(";").collect();
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for set in sets {
            let tokens: Vec<&str> = set.split_whitespace().collect();
            for x in tokens.chunks(2) {
                let num = x[0].parse::<u32>().unwrap();
                if x[1].contains("red") {
                    max_red = num.max(max_red);
                }
                if x[1].contains("blue") {
                    max_blue = num.max(max_blue);
                }
                if x[1].contains("green") {
                    max_green = num.max(max_green);
                }
            }
        }
        possible_game_ids.push(max_red * max_blue * max_green);
    }
    let s: u32 = possible_game_ids.into_iter().sum();
    s.to_string()
}
fn part1(input: &str) -> String {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;
    let mut possible_game_ids: Vec<u32> = Vec::new();
    'outer: for (i, line) in input.split("\n").filter(|x| !x.is_empty()).enumerate() {
        let line = line.to_string();
        let game_split: Vec<&str> = line.split(":").collect();
        let sets: Vec<&str> = game_split[1].split(";").collect();
        for set in sets {
            let tokens: Vec<&str> = set.split_whitespace().collect();
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for x in tokens.chunks(2) {
                let num = x[0].parse::<u32>().unwrap();
                if x[1].contains("red") {
                    if red + num > red_cubes {
                        continue 'outer;
                    }
                    red += num;
                }
                if x[1].contains("blue") {
                    if blue + num > blue_cubes {
                        continue 'outer;
                    }
                    blue += num;
                }
                if x[1].contains("green") {
                    if green + num > green_cubes {
                        continue 'outer;
                    }
                    green += num
                }
            }
        }
        possible_game_ids.push((i as u32) + 1);
    }
    let s: u32 = possible_game_ids.into_iter().sum();
    s.to_string()
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

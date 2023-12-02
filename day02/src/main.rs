use std::{
    env,
    io::{self, Read},
    process::exit,
};
#[derive(Debug)]
struct Cubes {
    color: String,
    num: u32,
}
fn part2(input: &str) -> String {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;
    let mut possible_game_ids: Vec<u32> = Vec::new();
    'outer: for (i, line) in input.split("\n").filter(|x| !x.is_empty()).enumerate() {
        let line = line.to_string();
        let game_split: Vec<&str> = line.split(":").collect();
        let sets: Vec<&str> = game_split[1].split(";").collect();
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for set in sets {
            let tokens: Vec<&str> = set.split_whitespace().collect();
            let mut red = Cubes {
                color: "red".to_string(),
                num: 0,
            };
            let mut blue = Cubes {
                color: "blue".to_string(),
                num: 0,
            };
            let mut green = Cubes {
                color: "green".to_string(),
                num: 0,
            };
            for x in tokens.chunks(2) {
                let num = x[0].parse::<u32>().unwrap();
                if x[1].contains("red") {
                    if num > max_red {
                        max_red = num;
                    }
                    red.num += num;
                }
                if x[1].contains("blue") {
                    if num > max_blue {
                        max_blue = num;
                    }
                    blue.num += num;
                }
                if x[1].contains("green") {
                    if num > max_green {
                        max_green = num;
                    }
                    green.num += num
                }
            }
        }
        possible_game_ids.push(max_red * max_blue * max_green);
    }
    dbg!(&possible_game_ids);
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
            let mut red = Cubes {
                color: "red".to_string(),
                num: 0,
            };
            let mut blue = Cubes {
                color: "blue".to_string(),
                num: 0,
            };
            let mut green = Cubes {
                color: "green".to_string(),
                num: 0,
            };
            for x in tokens.chunks(2) {
                let num = x[0].parse::<u32>().unwrap();
                if x[1].contains("red") {
                    if red.num + num > red_cubes {
                        continue 'outer;
                    }
                    red.num += num;
                }
                if x[1].contains("blue") {
                    if blue.num + num > blue_cubes {
                        continue 'outer;
                    }
                    blue.num += num;
                }
                if x[1].contains("green") {
                    if green.num + num > green_cubes {
                        continue 'outer;
                    }
                    green.num += num
                }
            }
        }
        possible_game_ids.push((i as u32) + 1);
    }
    dbg!(&possible_game_ids);
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
    dbg!(&input);
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

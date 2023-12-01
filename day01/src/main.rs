use std::{
    env,
    io::{self, Read},
    process::exit,
};
#[derive(Debug)]
struct NumMapping {
    str_repr: String,
    num: u32,
}
fn part2(input: &str) -> String {
    let mut count = 0;
    let mut num_map: Vec<NumMapping> = Vec::new();
    let str_repr_arr = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, str_repr) in str_repr_arr.iter().enumerate() {
        num_map.push(NumMapping {
            str_repr: str_repr.to_string(),
            num: (i as u32) + 1,
        });
    }
    for line in input.split("\n").filter(|x| !x.is_empty()) {
        let mut nums: Vec<char> = Vec::new();
        let chars = line.chars();
        for (i, ch) in chars.enumerate() {
            if let Some(_) = ch.to_digit(10) {
                nums.push(ch);
                continue;
            }
            for j in 2..=4 {
                if i + j >= line.len() {
                    break;
                }
                if str_repr_arr.contains(&&line[i..=i + j]) {
                    let x: Vec<&NumMapping> = num_map
                        .iter()
                        .filter(|x| x.str_repr == line[i..=i + j])
                        .collect();
                    nums.push(std::char::from_digit(x.first().unwrap().num, 10).unwrap());
                }
            }
        }
        let mut temp = String::new();
        temp.push(*nums.first().unwrap());
        temp.push(*nums.last().unwrap());
        count += temp.parse::<u32>().unwrap();
    }
    count.to_string()
}
fn part1(input: &str) -> String {
    let mut count = 0;
    for line in input.split("\n").filter(|x| !x.is_empty()) {
        let mut temp = String::new();
        let line: Vec<char> = line
            .chars()
            .into_iter()
            .filter(|x| x.to_digit(10).is_some())
            .collect();
        temp.push(*line.first().unwrap());
        temp.push(*line.last().unwrap());
        count += temp.parse::<u32>().unwrap();
    }
    count.to_string()
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

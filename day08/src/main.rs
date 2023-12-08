use std::{
    env,
    io::{self, Read},
    process::exit,
};
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}
fn part2(input: &str) -> String {
    let lr: Vec<char> = input.split("\n").nth(0).unwrap().chars().collect();
    let mut nodes = Vec::new();
    let mut ending_as = Vec::new();
    for line in lines(input).into_iter().skip(1) {
        let split: Vec<&str> = line.split(" = ").collect();
        let key = split[0].to_string();
        let val: Vec<&str> = split[1].split(", ").collect();
        let left = val[0].replace("(", "");
        let right = val[1].replace(")", "");
        if key.chars().last().unwrap() == 'A' {
            ending_as.push(key.clone());
        }
        nodes.push((key, (left, right)));
    }
    let mut res = Vec::new();
    for a in ending_as {
        let mut steps: u64 = 0;
        let mut target = a;
        'outer: for instruction in lr.iter().cycle() {
            for node in &nodes {
                if node.0 == target {
                    if node.0.chars().last().unwrap() == 'Z' {
                        break 'outer;
                    }
                    steps += 1;
                    match instruction {
                        'L' => target = String::from(node.1 .0.clone()),
                        'R' => target = String::from(node.1 .1.clone()),
                        _ => panic!("invalid instruction"),
                    }
                    break;
                }
            }
        }
        res.push(steps);
    }
    let mut ans = *res.get(0).unwrap();
    for l in res {
        if ans == l {
            continue;
        }
        ans = lcm(ans, l);
    }
    ans.to_string()
}
fn part1(input: &str) -> String {
    let lr: Vec<char> = input.split("\n").nth(0).unwrap().chars().collect();
    let mut nodes = Vec::new();
    for line in lines(input).into_iter().skip(1) {
        let split: Vec<&str> = line.split(" = ").collect();
        let key = split[0];
        let val: Vec<&str> = split[1].split(", ").collect();
        let left = val[0].replace("(", "");
        let right = val[1].replace(")", "");
        nodes.push((key, (left, right)));
    }
    // println!("{:?}", nodes);
    let mut steps = 0;
    let mut target = "AAA".to_string();
    'outer: for instruction in lr.into_iter().cycle() {
        for node in &nodes {
            if node.0 == target {
                if node.0 == "ZZZ" {
                    break 'outer;
                }
                steps += 1;
                match instruction {
                    'L' => target = String::from(node.1 .0.clone()),
                    'R' => target = String::from(node.1 .1.clone()),
                    _ => panic!("invalid instruction"),
                }
                break;
            }
        }
    }
    steps.to_string()
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

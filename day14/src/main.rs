use std::{
    collections::VecDeque,
    env,
    io::{self, Read},
    process::exit,
};
#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u32,
}
fn part2(input: &str) -> String {
    let sequences: Vec<&str> = input.split(",").collect();
    let mut results = 0;
    let mut boxes: Vec<VecDeque<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(VecDeque::new());
    }
    for sequence in sequences {
        let sequence = sequence.trim();
        let sp: Vec<&str> = sequence.split("=").collect();
        println!("{:?}", sp);
        if sp.len() == 2 {
            let label = sp[0];
            let focal = sp[1].parse::<u32>().unwrap();
            let vec = &mut boxes[hash(label)];
            let idx = vec.iter().position(|x| x.label == label);
            match idx {
                Some(idx) => vec[idx].focal_len = focal,
                None => vec.push_back(Lens {
                    label,
                    focal_len: focal,
                }),
            };
        } else {
            let label = sp[0];
            // strip off -
            let label = &label[0..label.len() - 1];
            let vec = &mut boxes[hash(label)];
            let idx = vec.iter().position(|x| x.label == label);
            match idx {
                Some(idx) => vec.remove(idx),
                None => continue,
            };
        }
        // end in -, split off -
    }
    for (i, b) in boxes.iter().enumerate() {
        if b.is_empty() {
            continue;
        }
        results += b
            .iter()
            .enumerate()
            .map(|lens| (i + 1) * (lens.0 + 1) * (lens.1.focal_len as usize))
            .fold(0, |acc, res| acc + res);
    }
    results.to_string()
}
fn part1(input: &str) -> String {
    let sequences: Vec<&str> = input.split(",").collect();
    let mut results = 0;
    for sequence in sequences {
        let curr_val = hash(sequence);
        results += curr_val;
    }
    results.to_string()
}
fn hash(s: &str) -> usize {
    let mut curr_val = 0u32;
    for c in s.as_bytes() {
        if *c == 10 || *c == 13 {
            continue;
        }
        curr_val += *c as u32;
        curr_val *= 17;
        curr_val %= 256;
    }
    curr_val as usize
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
    // dbg!(start.elapsed());
}

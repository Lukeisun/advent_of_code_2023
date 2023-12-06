use std::{
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    let rate = 1;
    let mut prod = 1;
    for lines in lines(input).chunks(2) {
        let time: Vec<&str> = lines[0]
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect();
        let time = time.join("");
        let distance: Vec<&str> = lines[1]
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect();
        let distance = distance.join("");
        let time = time.parse::<u64>().unwrap();
        let distance = distance.parse::<u64>().unwrap();
        let mut ways_to_win = 0;
        for i in 0..=time {
            let dist = i * rate * (time - i);
            if dist > distance {
                ways_to_win += 1;
            }
        }
        prod *= ways_to_win;
    }
    // for (time, distance) in times.into_iter().zip(distances) {
    //     // println!("{} {}", time, distance);
    // }
    prod.to_string()
}
fn part1(input: &str) -> String {
    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();
    for lines in lines(input).chunks(2) {
        let _ = lines[0]
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .for_each(|x| times.push(x));
        let _ = lines[1]
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .for_each(|x| distances.push(x));
    }
    let rate = 1;
    let mut prod = 1;
    for (time, distance) in times.into_iter().zip(distances) {
        // println!("{} {}", time, distance);
        let mut ways_to_win = 0;
        for i in 0..=time {
            let dist = i * rate * (time - i);
            if dist > distance {
                ways_to_win += 1;
            }
        }
        prod *= ways_to_win;
    }
    prod.to_string()
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

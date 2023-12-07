use std::{
    cmp::Ordering,
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    todo!();
}
fn label_num(label: char) -> u32 {
    if label.is_digit(10) {
        return label.to_digit(10).unwrap();
    }
    match label {
        'T' => 10,
        'J' => 25,
        'Q' => 50,
        'K' => 75,
        'A' => 100,
        _ => {
            panic!("Incorrect label provided");
        }
    }
}
fn part1(input: &str) -> String {
    let mut map = HashMap::new();
    for line in lines(input) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let hand = split[0];
        let bid = split[1].parse::<u32>();
        map.insert(hand, bid);
    }
    let mut kinds = HashMap::new();
    for (hand, _) in &map {
        println!("{hand}");
        let mut labels = HashMap::new();
        for label in hand.chars() {
            let c = labels.entry(label).or_insert(0);
            *c += 1;
        }
        // five = 7, four = 6, full = 5, three = 4, two = 3, one = 2, high = 1
        // println!("{:#?}", labels);
        match labels.len() {
            5 => {
                let vec = kinds.entry(1).or_insert(Vec::new());
                vec.push(hand);
            }
            4 => {
                let vec = kinds.entry(2).or_insert(Vec::new());
                vec.push(hand);
            }
            3 => {
                let ones = labels.into_values().filter(|x| *x == 1).count();
                if ones == 1 {
                    let vec = kinds.entry(3).or_insert(Vec::new());
                    vec.push(hand);
                } else {
                    let vec = kinds.entry(4).or_insert(Vec::new());
                    vec.push(hand);
                }
            }
            2 => {
                let min = labels.into_values().min().unwrap();
                if min == 1 {
                    let vec = kinds.entry(6).or_insert(Vec::new());
                    vec.push(hand);
                } else {
                    let vec = kinds.entry(5).or_insert(Vec::new());
                    vec.push(hand);
                }
            }
            1 => {
                let vec = kinds.entry(7).or_insert(Vec::new());
                vec.push(hand);
            }
            _ => panic!("error"),
        }
    }
    let mut kind_vec: Vec<(i32, Vec<&&str>)> = kinds.into_iter().collect();
    kind_vec.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ranks: [String; 5] = Default::default();
    let mut kind_vec_iter = kind_vec.into_iter();
    let mut rank_idx = 0;
    while let Some(kind) = kind_vec_iter.next() {
        let mut kind = kind.1;
        kind.sort_by(|a, b| {
            let a_val = a
                .chars()
                .into_iter()
                .map(|x| label_num(x))
                .fold(1u32, |acc, x| acc * x);
            let b_val = b
                .chars()
                .into_iter()
                .map(|x| label_num(x))
                .fold(1u32, |acc, x| acc * x);
            println!("a {:?} - {a_val} b {:?} -  {b_val}", a, b);
            if a_val > b_val {
                return Ordering::Less;
            } else {
                Ordering::Greater
            }
        });
        println!("{:?}", kind);
    }
    "pog".to_string()
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

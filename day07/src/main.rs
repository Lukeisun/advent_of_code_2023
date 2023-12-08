use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};
fn joker_label_num(label: char) -> u32 {
    if label.is_digit(10) {
        return label.to_digit(10).unwrap();
    }
    match label {
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            panic!("Incorrect label provided");
        }
    }
}
fn label_num(label: char) -> u32 {
    if label.is_digit(10) {
        return label.to_digit(10).unwrap();
    }
    match label {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            panic!("Incorrect label provided");
        }
    }
}
fn part2(input: &str) -> String {
    let mut map = HashMap::new();
    for line in lines(input) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let hand = split[0];
        let bid = split[1].parse::<u32>().unwrap();
        map.insert(hand, bid);
    }
    let mut kinds = HashMap::new();
    for (hand, _) in &map {
        let mut labels = HashMap::new();
        for label in hand.chars() {
            let c = labels.entry(label).or_insert(0);
            *c += 1;
        }
        // five = 7, four = 6, full = 5, three = 4, two = 3, one = 2, high = 1
        let mut js = 0;
        if let Some(c) = labels.get(&'J') {
            js = *c;
        }
        let hand = String::from_iter(hand.chars());
        match labels.len() {
            5 => {
                // high
                //J3456 1 only care if 1 j, if so becomes one pair
                let mut entry = 1;
                if js == 1 {
                    entry = 2;
                }
                let vec = kinds.entry(entry).or_insert(Vec::new());
                vec.push(hand);
            }
            4 => {
                // one
                let mut entry = 2;
                println!("{js}");
                if js == 1 || js == 2 {
                    entry = 4;
                }
                let vec = kinds.entry(entry).or_insert(Vec::new());
                vec.push(hand);
            }
            3 => {
                let ones = labels.into_values().filter(|x| *x == 1).count();
                if ones == 1 {
                    // two
                    // 23432 1 two pair
                    // J343J 1 four of a kind
                    // 2J4J2 1 four of a kind
                    // 23J32 1 full house
                    let mut entry = 3;
                    if js == 2 {
                        entry = 6;
                    } else if js == 1 {
                        entry = 5
                    }
                    let vec = kinds.entry(entry).or_insert(Vec::new());
                    vec.push(hand);
                } else {
                    // three
                    let mut entry = 4;
                    if js == 1 {
                        entry = 6;
                    } else if js == 2 {
                        entry = 7;
                    }
                    let vec = kinds.entry(entry).or_insert(Vec::new());
                    vec.push(hand);
                }
            }
            2 => {
                let min = labels.into_values().min().unwrap();
                if min == 1 {
                    // four
                    let mut entry = 6;
                    if js > 0 {
                        entry = 7;
                    }
                    let vec = kinds.entry(entry).or_insert(Vec::new());

                    vec.push(hand);
                } else {
                    // full
                    let mut vec = kinds.entry(5).or_insert(Vec::new());
                    if js > 0 {
                        vec = kinds.entry(7).or_insert(Vec::new());
                    }
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
    let mut kind_vec: Vec<(i32, Vec<String>)> = kinds.into_iter().collect();
    kind_vec.sort_by(|a, b| a.0.cmp(&b.0));
    // five = 7, four = 6, full = 5, three = 4, two = 3, one = 2, high = 1
    println!("{:?}", kind_vec);
    let mut ranks = Vec::new();
    for (_, kind) in kind_vec.clone() {
        println!("ranks {:?}", ranks);
        println!("kind {:?}", kind);
        let mut kind = kind.clone();
        kind.sort_by(|a, b| {
            for (c, o) in a.chars().zip(b.chars()) {
                println!("c - {c}, o - {o}");
                if c == o {
                    continue;
                }
                return joker_label_num(c).cmp(&joker_label_num(o));
            }
            std::cmp::Ordering::Equal
        });
        println!("kind {:?}", kind);
        kind.into_iter().for_each(|x| ranks.push(x));
    }
    println!("fin ranks {:?}", ranks);
    println!("{:#?}", kind_vec);
    let mut total_winnings = 0;
    for (i, hand) in ranks.into_iter().enumerate() {
        total_winnings += (i + 1) as u32 * map.get(&(hand.as_str())).copied().unwrap();
    }
    total_winnings.to_string()
}
fn part1(input: &str) -> String {
    let mut map = HashMap::new();
    for line in lines(input) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let hand = split[0];
        let bid = split[1].parse::<u32>().unwrap();
        map.insert(hand, bid);
    }
    let mut kinds = HashMap::new();
    for (hand, _) in &map {
        let mut labels = HashMap::new();
        for label in hand.chars() {
            let c = labels.entry(label).or_insert(0);
            *c += 1;
        }
        // five = 7, four = 6, full = 5, three = 4, two = 3, one = 2, high = 1
        let hand = String::from_iter(hand.chars());
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
    let mut kind_vec: Vec<(i32, Vec<String>)> = kinds.into_iter().collect();
    kind_vec.sort_by(|a, b| a.0.cmp(&b.0));
    // five = 7, four = 6, full = 5, three = 4, two = 3, one = 2, high = 1
    println!("{:?}", kind_vec);
    let mut ranks = Vec::new();
    for (_, kind) in kind_vec {
        println!("ranks {:?}", ranks);
        println!("kind {:?}", kind);
        let mut kind = kind.clone();
        kind.sort_by(|a, b| {
            for (c, o) in a.chars().zip(b.chars()) {
                println!("c - {c}, o - {o}");
                if c == o {
                    continue;
                }
                return label_num(c).cmp(&label_num(o));
            }
            std::cmp::Ordering::Equal
        });
        println!("kind {:?}", kind);
        kind.into_iter().for_each(|x| ranks.push(x));
    }
    println!("fin ranks {:?}", ranks);
    let mut total_winnings = 0;
    for (i, hand) in ranks.into_iter().enumerate() {
        total_winnings += (i + 1) as u32 * map.get(&(hand.as_str())).copied().unwrap();
    }
    total_winnings.to_string()
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

use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in lines(input) {
        let mut winning_cards = HashMap::new();
        let game: Vec<&str> = line.split(":").collect();
        let card_pairs: Vec<&str> = game[1].split("|").collect();
        let _ = card_pairs[0].split_whitespace().into_iter().for_each(|x| {
            winning_cards.insert(x.parse::<u32>().unwrap(), ());
        });
        let filtered: Vec<u32> = card_pairs[1]
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|x| winning_cards.contains_key(&x))
            .collect();
        let filtered_count = filtered.into_iter().count() as u32;
        if filtered_count > 0 {
            sum += 2u32.pow(filtered_count - 1);
        }
    }
    sum.to_string()
}
fn part2(input: &str) -> String {
    let mut sum = 0;
    let mut card_counts: Vec<u32> = Vec::new();
    for line in lines(input) {
        let mut winning_cards = HashMap::new();
        let game: Vec<&str> = line.split(":").collect();
        let card_pairs: Vec<&str> = game[1].split("|").collect();
        let _ = card_pairs[0].split_whitespace().into_iter().for_each(|x| {
            winning_cards.insert(x.parse::<u32>().unwrap(), ());
        });
        let filtered: Vec<u32> = card_pairs[1]
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|x| winning_cards.contains_key(&x))
            .collect();
        let filtered_count = filtered.into_iter().count() as u32;
        card_counts.push(filtered_count);
    }
    let mut map = HashMap::new();
    for (i, _) in card_counts.clone().into_iter().enumerate() {
        map.insert(i as u32 + 1, 1);
    }
    for (i, card) in card_counts.clone().into_iter().enumerate() {
        for j in 1..=card {
            let cur = i as u32 + 1;
            if map.contains_key(&(cur + j)) {
                let instances = *map.get(&cur).unwrap();
                *map.entry(cur + j).or_insert(0) += 1 * instances;
            }
        }
    }
    // println!("{:#?}", map);
    map.into_values().sum::<u32>().to_string()
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

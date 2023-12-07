use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    ops::Range,
    process::exit,
    u32,
};
fn part2(input: &str) -> String {
    // the destination range start, the source range start, and the range length.
    let blocks: Vec<&str> = input.split("\n").collect();
    let s: Vec<u64> = blocks[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut seeds: Vec<Range<u64>> = Vec::new();
    for (i, seed) in s.clone().into_iter().enumerate().step_by(2) {
        seeds.push(Range {
            start: seed,
            end: seed + *s.get(i + 1).unwrap(),
        });
    }
    // println!("{:#?}", seeds);
    let mut u_to_v_map: Vec<HashMap<Range<u64>, Range<u64>>> = Vec::new();
    let mut map = HashMap::new();
    for block in blocks.into_iter().skip(1) {
        if block.is_empty() {
            if map.is_empty() {
                continue;
            }
            u_to_v_map.push(map);
            map = HashMap::new();
            continue;
        }
        if block.contains(":") {
            continue;
        }
        let nums: Vec<u64> = block
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let dest_start = nums[0];
        let src_start = nums[1];
        let range_length = nums[2];
        let dest_range = Range {
            start: dest_start,
            end: dest_start + range_length,
        };
        let src_range = Range {
            start: src_start,
            end: src_start + range_length,
        };
        map.insert(src_range, dest_range);
    }
    println!("{:?}", u_to_v_map);
    // let mut ranges: Vec<Range<u64>> = seeds.clone();
    let mut ranges = HashMap::new();
    for seed in seeds {
        ranges.insert(seed, ());
    }
    for map in u_to_v_map.into_iter() {
        // println!("{:?}", map);
        let mut temp = HashMap::new();
        // println!("{:?}", ranges);
        for (range, _) in &ranges {
            let mut handled = false;
            for (src, dest) in &map {
                let contains_start = src.contains(&range.start);
                let contains_end = src.contains(&(range.end - 1));
                if contains_end && contains_start {
                    dbg!("BOTH CONTAINS");
                    println!("src: {:?} - dest: {:?} - range - {:?}", src, dest, range);
                    println!("{:?}", temp);
                    let start_diff = range.start - src.start;
                    let range_diff = range.end - range.start;
                    println!(
                        "{:?} range_diff {} start_diff {}",
                        src.start + start_diff..src.start + start_diff + range_diff,
                        range_diff,
                        start_diff
                    );
                    temp.insert(
                        dest.start + start_diff..dest.start + start_diff + range_diff,
                        (),
                    );
                    println!("{:?}", temp);
                    handled = true;
                } else if contains_end && !contains_start {
                    dbg!("END CONTAINS");
                    temp.insert(range.start..src.start, ());
                    let diff = range.end - src.start;
                    temp.insert(dest.start..dest.start + diff, ());
                    handled = true;
                } else if contains_start {
                    dbg!("START CONTAINS");
                    temp.insert(src.end..range.end, ());
                    let start_diff = range.start - src.start;
                    temp.insert(dest.start + start_diff..dest.end, ());
                    handled = true;
                }
            }
            if !handled {
                temp.insert(range.clone(), ());
            }
        }
        ranges = temp;
        // println!("{:#?}", ranges);
    }
    let mut ranges: Vec<u64> = ranges.into_keys().map(|x| x.start as u64).collect();
    ranges.sort();
    println!("{:#?}\n{}", ranges, ranges[0]);
    "pog".to_string()
}
fn part1(input: &str) -> String {
    // the destination range start, the source range start, and the range length.
    let blocks: Vec<&str> = input.split("\n").collect();
    let seeds: Vec<u64> = blocks[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    println!("{:?}", seeds);
    let mut u_to_v_map: Vec<(String, HashMap<Range<u64>, Range<u64>>)> = Vec::new();
    let mut u_to_v = String::new();
    let mut map = HashMap::new();
    for block in blocks.into_iter().skip(1) {
        if block.is_empty() {
            if map.is_empty() {
                continue;
            }
            u_to_v_map.push((u_to_v.clone(), map));
            map = HashMap::new();
            continue;
        }
        if block.contains(":") {
            u_to_v = block.split_whitespace().nth(0).unwrap().to_string();
            continue;
        }
        let nums: Vec<u64> = block
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let dest_start = nums[0];
        let src_start = nums[1];
        let range_length = nums[2];
        let dest_range = Range {
            start: dest_start,
            end: dest_start + range_length,
        };
        let src_range = Range {
            start: src_start,
            end: src_start + range_length,
        };
        map.insert(src_range, dest_range);
    }
    let u_to_v_map = &*u_to_v_map;
    let mut locations = Vec::new();
    let u_to_v_map = &*u_to_v_map;
    for seed in seeds {
        let mut source = seed;
        for m in u_to_v_map {
            let curr_map = &m.1;
            let range = curr_map
                .into_iter()
                .filter(|x| x.0.contains(&source))
                .map(|x| x.0)
                .reduce(|_, range| range);
            if let Some(range) = range {
                let diff = source - range.start;
                let dest = curr_map.get(&range).unwrap();
                source = dest.start + diff;
            }
        }
        locations.push(source);
    }
    locations.into_iter().min().unwrap().to_string()
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

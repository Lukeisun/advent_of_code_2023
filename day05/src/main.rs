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
    // println!("{:#?}", u_to_v_map);
    let u_to_v_map = &*u_to_v_map;
    let mut vec = Vec::new();
    for seed in seeds {
        let mut ranges = Vec::new();
        ranges.push(seed);
        for map in u_to_v_map.into_iter() {
            let mut applied = Vec::new();
            for (src, dst) in map {
                let mut temp = Vec::new();
                while ranges.len() > 0 {
                    let curr = ranges.pop().unwrap();
                    let before = curr.start..curr.end.min(src.start);
                    let inter = curr.start.max(src.start)..src.end.min(curr.end);
                    let after = src.end.max(curr.start)..curr.end;
                    if before.end > before.start {
                        temp.push(before);
                    }
                    if inter.end > inter.start {
                        applied.push(
                            inter.start - src.start + dst.start..inter.end - src.start + dst.start,
                        );
                    }
                    if after.end > after.start {
                        temp.push(after);
                    }
                }
                ranges = temp;
            }
            ranges.append(&mut applied);
        }
        let min = ranges.into_iter().map(|x| x.start).min().unwrap();
        vec.push(min);
    }
    println!("{:?}", vec);
    vec.into_iter().min().unwrap().to_string()
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

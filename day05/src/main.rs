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
    println!("{:#?}", u_to_v_map);
    let mut seed_to: Vec<(Range<u64>, Range<u64>)> =
        seeds.clone().into_iter().map(|x| (x.clone(), x)).collect();
    let u_to_v_map = &*u_to_v_map;
    for map in u_to_v_map.into_iter().take(2) {
        let mut temp = Vec::new();
        println!("{:?}", map);
        for (seed, to) in seed_to.clone() {
            println!("\nseed: {:?}\nto: {:?}\n", seed, to);
            for (key, val) in &map.1 {
                let contains_start = key.contains(&to.start);
                let contains_end = key.contains(&(to.end - 1));
                println!("\nkey {:?}\nval {:?}", key, val);
                if contains_start && contains_end {
                    dbg!("BOTH CONTAINS");
                    let start_diff = to.start - key.start;
                    let end_diff = key.end - to.end;
                    let new_to_val = val.start + start_diff..val.end - end_diff;
                    let new_to = (seed.clone(), new_to_val);
                    println!("{:?}", new_to);
                    temp.push(new_to);
                    let left_potential_seed = key.start..to.start;
                    let left_potential = (left_potential_seed, val.start..val.start + start_diff);
                    // temp.push(left_potential);
                    let right_potential_seed = to.end..key.end;
                    let right_potential = (right_potential_seed, val.end - end_diff..val.end);
                    // temp.push(right_potential);
                } else if contains_end && !contains_start {
                    dbg!("END CONTAINS");
                    // src/dst = key/val
                    // seed/val = seed/to
                    let split = key.start;
                    // seed range
                    let f_val_split = to.start..split;
                    // new range
                    let s_val_split = split..to.end;
                    println!("val {:?} {:?}", f_val_split, s_val_split);
                    let diff = s_val_split.end - s_val_split.start;
                    let seed_split = seed.start + (split - val.start);
                    let f_seed_split = seed.start..seed_split;
                    let s_seed_split = seed_split..seed.end;
                    let first = (f_seed_split, f_val_split);
                    println!("first {:?}", first);
                    temp.push(first);
                    let second = (s_seed_split, val.start..val.start + diff);
                    println!("second {:?}", second);
                    temp.push(second);
                // let diff = .end - split;
                // let original =
                } else if contains_start {
                    dbg!("START CONTAINS");
                } else {
                    dbg!("NO CONTAINS");
                    temp.push((key.clone(), val.clone()));
                }
            }
        }
        seed_to = temp;
        println!("{:#?}", seed_to);
    }
    // seeds.push(13..13);
    // for m in u_to_v_map {
    //     let mut temp = Vec::new();
    //     println!("m {:?}", m);
    //     println!("{:?}", seeds_unravel);
    //     for s in &seeds_unravel {
    //         let seed = &s.0;
    //         let value = &s.1;
    //         println!("seed: {:?}\nvalue: {:?}", seed, value);
    //         let curr_map = &m.1;
    //         let full_contains = curr_map
    //             .into_iter()
    //             .filter(|x| x.0.contains(&value.end) && x.0.contains(&value.start))
    //             .map(|x| x)
    //             .reduce(|_, range| range);
    //         }
    //         let start_contains = curr_map
    //             .into_iter()
    //             .filter(|x| !x.0.contains(&value.end) && x.0.contains(&value.start))
    //             .map(|x| x)
    //             .reduce(|_, range| range);
    //         if let Some(mapping) = start_contains {
    //             println!("START");
    //             let src_range = mapping.0;
    //             let dst_range = mapping.1;
    //             let split = src_range.end;
    //             println!("src {:?} dst {:?} {split}", src_range, dst_range);
    //             let first_value_split = Range {
    //                 start: value.start,
    //                 end: split,
    //             };
    //             let rest_value_split = Range {
    //                 start: split,
    //                 end: value.end,
    //             };
    //             let diff = first_value_split.end - first_value_split.start;
    //             let seed_split = seed.start + diff;
    //             let first_seed_split = Range {
    //                 start: seed.start,
    //                 end: seed_split,
    //             };
    //             let rest_seed_split = Range {
    //                 start: seed_split,
    //                 end: seed.end,
    //             };
    //             let rest_split = (rest_seed_split, rest_value_split);
    //             let first_split = (
    //                 first_seed_split,
    //                 Range {
    //                     start: dst_range.end - diff,
    //                     end: dst_range.end,
    //                 },
    //             );
    //             // let more_seeds = (
    //             //     Range {
    //             //         start: src_range.start,
    //             //         end: seed.start,
    //             //     },
    //             //     Range {
    //             //         start: dst_range.start,
    //             //         end: dst_range.start + (seed.start - src_range.start),
    //             //     },
    //             // );
    //             println!(
    //                 "first {:?} second {:?} diff {:?} ",
    //                 first_split, rest_split, diff
    //             );
    //             // temp.push(more_seeds);
    //             temp.push(first_split);
    //             temp.push(rest_split);
    //             continue;
    //         }
    //         let end_contains = curr_map
    //             .into_iter()
    //             .filter(|x| x.0.contains(&value.end) && !x.0.contains(&value.start))
    //             .map(|x| x)
    //             .reduce(|_, range| range);
    //         if let Some(mapping) = end_contains {
    //             let src_range = mapping.0;
    //             let dst_range = mapping.1;
    //             let split = src_range.start;
    //             let first_value_split = Range {
    //                 start: value.start,
    //                 end: split,
    //             };
    //             let second_value_split = Range {
    //                 start: split,
    //                 end: value.end,
    //             };
    //             let diff = second_value_split.end - second_value_split.start;
    //             let seed_split = seed.end - diff;
    //             let first_seed_split = Range {
    //                 start: seed.start,
    //                 end: seed_split,
    //             };
    //             let second_seed_split = Range {
    //                 start: seed_split,
    //                 end: seed.end,
    //             };
    //             let first_split = (first_seed_split, first_value_split);
    //             let second_split = (
    //                 second_seed_split,
    //                 Range {
    //                     start: dst_range.start,
    //                     end: dst_range.start + diff,
    //                 },
    //             );
    //             temp.push(first_split);
    //             temp.push(second_split);
    //             continue;
    //         }
    //         println!("s - {:?}", s);
    //         temp.push(s.clone());
    //     }
    //     println!("temp {:?}", temp);
    //     seeds_unravel = temp;
    //     println!("seeds {:#?}", seeds_unravel.clone());
    // }
    // println!("{:#?}", seeds_unravel);
    "pog".to_string()
    // let mut locations = Vec::new();
    // for s in seeds {
    //     for seed in s {
    //         let mut source = seed;
    //         for m in u_to_v_map {
    //             let curr_map = &m.1;
    //             let range = curr_map
    //                 .into_iter()
    //                 .filter(|x| x.0.contains(&source))
    //                 .map(|x| x.0)
    //                 .reduce(|_, range| range);
    //             if let Some(range) = range {
    //                 let diff = source - range.start;
    //                 let dest = curr_map.get(&range).unwrap();
    //                 source = dest.start + diff;
    //             }
    //         }
    //         locations.push(source);
    //     }
    // }
    // locations.into_iter().min().unwrap().to_string()
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

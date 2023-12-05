use fasthash::spooky::Hash64;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    env,
    io::{self, Read},
    ops::{Range, RangeInclusive},
    process::exit,
    u32,
    usize::MAX,
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
    let mut u_to_v_map = HashMap::new();
    let mut u = String::new();
    let mut v = String::new();
    let mut map = HashMap::new();
    for block in blocks.into_iter().skip(1) {
        if block.is_empty() {
            if map.is_empty() {
                continue;
            }
            u_to_v_map.insert((u.clone(), v.clone()), map);
            map = HashMap::new();
            continue;
        }
        if block.contains(":") {
            let u_to_v: Vec<&str> = block
                .split_whitespace()
                .nth(0)
                .unwrap()
                .split("-to-")
                .collect();
            u = u_to_v[0].to_string();
            v = u_to_v[1].to_string();
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
    let seed_s = "seed".to_string();
    let soil = "soil".to_string();
    let fertilizer = "fertilizer".to_string();
    let water = "water".to_string();
    let light = "light".to_string();
    let temperature = "temperature".to_string();
    let humidity = "humidity".to_string();
    let location = "location".to_string();
    // println!("{:#?}", u_to_v_map);
    let seed_soil = u_to_v_map.get(&(seed_s.clone(), soil.clone())).unwrap();
    let soil_fert = u_to_v_map.get(&(soil.clone(), fertilizer.clone())).unwrap();
    let fert_water = u_to_v_map
        .get(&(fertilizer.clone(), water.clone()))
        .unwrap();
    let water_light = u_to_v_map.get(&(water.clone(), light.clone())).unwrap();
    let light_temperature = u_to_v_map
        .get(&(light.clone(), temperature.clone()))
        .unwrap();
    let temperature_humidtiy = u_to_v_map
        .get(&(temperature.clone(), humidity.clone()))
        .unwrap();
    let humidity_location = u_to_v_map
        .get(&(humidity.clone(), location.clone()))
        .unwrap();
    let arr = [
        seed_soil,
        soil_fert,
        fert_water,
        water_light,
        light_temperature,
        temperature_humidtiy,
        humidity_location,
    ];
    let mut locations = Vec::new();
    for s in seeds {
        for seed in s {
            // println!("{seed}");
            let mut source = seed;
            let mut curr_range: Vec<Range<u64>> = Vec::new();
            for m in arr {
                curr_range = m
                    .clone()
                    .into_keys()
                    .filter(|x| x.contains(&source))
                    .collect();
                if !curr_range.is_empty() {
                    // println!("curr {:?}", curr_range[0]);
                    // println!("src b {source}");
                    let diff = source - curr_range[0].start;
                    let dest = m.get(&curr_range[0]).unwrap();
                    // println!("dest {:?}", dest);
                    source = dest.start + diff;
                    // println!("diff {} src after {}", diff, source);
                }
            }
            locations.push(source);
        }
    }
    locations.into_iter().min().unwrap().to_string()
    // "pog".to_string()
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
    let mut u_to_v_map = HashMap::new();
    let mut u = String::new();
    let mut v = String::new();
    let mut map = HashMap::new();
    for block in blocks.into_iter().skip(1) {
        if block.is_empty() {
            if map.is_empty() {
                continue;
            }
            u_to_v_map.insert((u.clone(), v.clone()), map);
            map = HashMap::new();
            continue;
        }
        if block.contains(":") {
            let u_to_v: Vec<&str> = block
                .split_whitespace()
                .nth(0)
                .unwrap()
                .split("-to-")
                .collect();
            u = u_to_v[0].to_string();
            v = u_to_v[1].to_string();
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
    let seed_s = "seed".to_string();
    let soil = "soil".to_string();
    let fertilizer = "fertilizer".to_string();
    let water = "water".to_string();
    let light = "light".to_string();
    let temperature = "temperature".to_string();
    let humidity = "humidity".to_string();
    let location = "location".to_string();
    // println!("{:#?}", u_to_v_map);
    let seed_soil = u_to_v_map.get(&(seed_s.clone(), soil.clone())).unwrap();
    let soil_fert = u_to_v_map.get(&(soil.clone(), fertilizer.clone())).unwrap();
    let fert_water = u_to_v_map
        .get(&(fertilizer.clone(), water.clone()))
        .unwrap();
    let water_light = u_to_v_map.get(&(water.clone(), light.clone())).unwrap();
    let light_temperature = u_to_v_map
        .get(&(light.clone(), temperature.clone()))
        .unwrap();
    let temperature_humidtiy = u_to_v_map
        .get(&(temperature.clone(), humidity.clone()))
        .unwrap();
    let humidity_location = u_to_v_map
        .get(&(humidity.clone(), location.clone()))
        .unwrap();
    let arr = [
        seed_soil,
        soil_fert,
        fert_water,
        water_light,
        light_temperature,
        temperature_humidtiy,
        humidity_location,
    ];
    let mut locations = Vec::new();
    for seed in seeds {
        let mut source = seed;
        let mut curr_range: Vec<Range<u64>> = Vec::new();
        for m in arr {
            curr_range = m
                .clone()
                .into_keys()
                .filter(|x| x.contains(&source))
                .collect();
            if !curr_range.is_empty() {
                // println!("curr {:?}", curr_range[0]);
                // println!("src b {source}");
                let diff = source - curr_range[0].start;
                let dest = m.get(&curr_range[0]).unwrap();
                // println!("dest {:?}", dest);
                source = dest.start + diff;
                // println!("diff {} src after {}", diff, source);
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

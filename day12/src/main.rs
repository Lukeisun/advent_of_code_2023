use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in lines(input).into_iter() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let row = line[0];
        let og_arrangements: Vec<u32> = line[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        // let groups: Vec<&str> = row.split(".").filter(|x| !x.is_empty()).collect();
        // let groups: Vec<char> = groups.join(".").chars().collect();
        let groups: Vec<char> = row.chars().collect();
        let mut arrangements = Vec::new();
        let mut g = String::new();
        for _ in 0..5 {
            let z = groups.clone().into_iter().collect::<String>();
            arrangements.append(&mut og_arrangements.clone());
            g.push_str(&z);
            g.push('?');
        }
        // let groups = groups.join("").chars().collect();
        // println!(
        //     "groups {:?} arrangements {:?}",
        //     &g.chars().collect::<Vec<char>>(),
        //     arrangements
        // );
        println!("{g} arrangements {:?}", arrangements);
        let mut m = HashMap::new();
        let c = perm(&g.chars().collect(), &arrangements, 0, 0, &mut m);
        println!("{c}");
        let mut s: Vec<String> = m.into_keys().collect();
        // s.sort();
        // for z in &s {
        //     println!("{z}");
        // }
        assert!(s.into_iter().count() == c as usize);
        sum += c;
    }
    sum.to_string()
}
fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in lines(input).into_iter() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let row = line[0];
        let arrangements: Vec<u32> = line[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        // let groups: Vec<&str> = row.split(".").filter(|x| !x.is_empty()).collect();
        // let groups = groups.join(".").chars().collect();
        let groups: Vec<char> = row.chars().collect();
        // let groups = groups.join("").chars().collect();
        println!("groups {:?} arrangements {:?}", groups, arrangements);
        let mut m = HashMap::new();
        let c = perm(&groups, &arrangements, 0, 0, &mut m);
        println!("{c}");
        let mut s: Vec<String> = m.into_keys().collect();
        s.sort();
        for z in &s {
            println!("{z}");
        }
        assert!(s.into_iter().count() == c as usize);
        sum += c;
    }
    sum.to_string()
}
// 9093 too high i beleaf
fn perm(
    groups: &Vec<char>,
    arrangements: &Vec<u32>,
    k: usize,
    arra_idx: usize,
    m: &mut HashMap<String, ()>,
) -> u32 {
    if arra_idx == arrangements.len() {
        let mut counter = 0;
        // println!("{:?}", groups);
        let mut lens: Vec<u32> = Vec::new();
        for c in groups.iter() {
            if *c == '#' {
                counter += 1;
            }
            if *c != '#' && counter > 0 {
                lens.push(counter);
                counter = 0;
            }
        }
        if counter > 0 {
            lens.push(counter);
        }
        // println!("lens {:?}", lens);
        let t = lens.into_iter().eq(arrangements.clone().into_iter());
        if !t {
            // println!("hit");
            return 0;
        }
        let g = groups.into_iter().collect::<String>();
        if m.contains_key(&g) {
            panic!("duplicate");
            // return 0;
        }
        m.insert(g.clone(), ());
        return 1;
    }
    let mut ret = 0;
    'outer: for i in k..groups.len() {
        let len = arrangements[arra_idx];
        let mut groups = groups.clone();
        let next = i + len as usize;
        for j in i..next {
            if j >= groups.len() || groups[j] == '.' {
                continue 'outer;
            }
            groups[j] = '#';
            // if groups[j] == '?' {
            //     groups[j] = 'T';
            // }
        }
        let prev = i.checked_sub(1);
        if let Some(c) = groups.get(next) {
            if *c == '#' {
                continue 'outer;
            }
        }
        if let Some(prev) = prev {
            if let Some(c) = groups.get(prev) {
                if *c == '#' {
                    continue 'outer;
                }
            }
        }
        // println!(
        //     "groups0 {:?}",
        //     groups.clone().into_iter().collect::<String>()
        // );
        // println!("i {} len {}  next {}", i, len, next);
        ret += perm(&groups, arrangements, next + 1, arra_idx + 1, m);
    }
    ret
}
// operational (.) or damaged (#). This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.
// ? # ? # ? # ? # ? # ? # ? # ? 1,3,1,6
// ?
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

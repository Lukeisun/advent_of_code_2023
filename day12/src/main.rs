use std::{
    collections::HashMap,
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
    todo!();
    // let mut sum = 0;
    // for line in lines(input).into_iter() {
    //     let line: Vec<&str> = line.split_whitespace().collect();
    //     let row = line[0];
    //     let og_arrangements: Vec<u32> = line[1]
    //         .split(",")
    //         .map(|x| x.parse::<u32>().unwrap())
    //         .collect();
    //
    //     // let groups: Vec<&str> = row.split(".").filter(|x| !x.is_empty()).collect();
    //     // let groups: Vec<char> = groups.join(".").chars().collect();
    //     let groups: Vec<char> = row.chars().collect();
    //     let mut arrangements = Vec::new();
    //     let mut g = String::new();
    //     for _ in 0..5 {
    //         let z = groups.clone().into_iter().collect::<String>();
    //         arrangements.append(&mut og_arrangements.clone());
    //         g.push_str(&z);
    //         g.push('?');
    //     }
    //     // let groups = groups.join("").chars().collect();
    //     // println!(
    //     //     "groups {:?} arrangements {:?}",
    //     //     &g.chars().collect::<Vec<char>>(),
    //     //     arrangements
    //     // );
    //     println!("{g} arrangements {:?}", arrangements);
    //     let mut m = HashMap::new();
    //     let c = perm(&g.chars().collect(), &arrangements, 0, 0, &mut m);
    //     println!("{c}");
    //     let mut s: Vec<String> = m.into_keys().collect();
    //     // s.sort();
    //     // for z in &s {
    //     //     println!("{z}");
    //     // }
    //     assert!(s.into_iter().count() == c as usize);
    //     sum += c;
    // }
    // sum.to_string()
}
fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in lines(input).into_iter() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let row = line[0];
        let arrangements: Vec<usize> = line[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let board: Vec<char> = row.chars().collect();
        // let groups = groups.join("").chars().collect();
        println!("groups {:?} arrangements {:?}", board, arrangements);
        // let mut m = HashMap::new();
        let c = perm(&board, &arrangements, 0, 0, 0);
        println!("{c}");
        // let mut s: Vec<String> = m.into_keys().collect();
        // s.sort();
        // for z in &s {
        //     println!("{z}");
        // }
        // assert!(s.into_iter().count() == c as usize);
        sum += c;
    }
    sum.to_string()
}
fn perm(board: &Vec<char>, arrangements: &Vec<usize>, si, ai, counter) -> u64 {

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

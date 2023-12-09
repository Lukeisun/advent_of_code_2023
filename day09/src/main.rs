use std::{
    env,
    io::{self, Read},
    process::exit,
};

fn part2(input: &str) -> String {
         let mut sum = 0;
    for line in lines(input).into_iter(){
        let vals: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap() ).collect();
         println!("vals {:?}",vals); 
         let mut v = Vec::new();
         v.push(vals);
         while !v.iter().last().unwrap().iter().all(|x: &i32| *x == 0) {
             let diff: Vec<i32> = v.iter().last().unwrap().windows(2).
                 map(|chunk| chunk.to_vec())
                 .flat_map(|chunk| vec!(chunk[1] - chunk[0]))
                 .collect();
             println!("diff {:?}", diff);
             v.push(diff);
        }
         println!("fin v {:?}", v);
         let mut prev = 0;
         for x in v.into_iter().rev() {
             let first = x[0];
             prev = first - prev;
         println!("prev {:?}", prev);
         }
         sum += prev;
         println!("fin prev {:?}", prev);
    }
    sum.to_string()
}
fn part1(input: &str) -> String {
         let mut sum = 0;
    for line in lines(input).into_iter(){
        let vals: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap() ).collect();
         println!("vals {:?}",vals); 
         let mut v = Vec::new();
         v.push(vals);
         while !v.iter().last().unwrap().iter().all(|x: &i32| *x == 0) {
             let diff: Vec<i32> = v.iter().last().unwrap().windows(2).
                 map(|chunk| chunk.to_vec())
                 .flat_map(|chunk| vec!(chunk[1] - chunk[0]))
                 .collect();
             println!("diff {:?}", diff);
             v.push(diff);
        }
         println!("v {:?}", v);
         let mut prev = 0;
         for x in v.into_iter().rev() {
             let last = x.iter().last().unwrap();
             prev = last + prev;
         }
         sum += prev;
         println!("prev {:?}", prev);
    }
    sum.to_string()
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

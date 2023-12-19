use core::panic;
use std::{
    env,
    fmt::Debug,
    io::{self, Read},
    process::exit,
};
#[derive(Debug)]
struct Rule<'a> {
    part: &'a str,
    lt: bool,
    amt: u32,
    send_to: &'a str,
}
#[derive(Debug)]
struct Tape<'a> {
    workflow: &'a str,
    rules: Vec<Rule<'a>>,
    end: &'a str,
}
// could maybe bin search the range?
// find split point, add A's that way
fn part2(input: &str) -> String {
    let (workflows, _) = parse_input(input);
    println!("{:?}", workflows);
    "pog".to_string()
}
fn part1(input: &str) -> String {
    let (workflows, ratings) = parse_input(input);
    let first = workflows.iter().find(|x| x.workflow == "in");
    let mut a = 0;
    // x m a s
    for rating in ratings.chunks(4) {
        let mut curr_workflow = first.unwrap();
        'outer: loop {
            for rule in &curr_workflow.rules {
                let idx = match rule.part {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => panic!(),
                };
                let cls: Box<dyn Fn(u32) -> bool> = match rule.lt {
                    true => Box::new(|x| x < rule.amt),
                    false => Box::new(|x| x > rule.amt),
                };
                if cls(rating[idx].1) {
                    if rule.send_to == "A" || rule.send_to == "R" {
                        match rule.send_to {
                            "A" => a += rating.iter().map(|x| x.1).sum::<u32>(),
                            "R" => (),
                            _ => panic!(),
                        };
                        break 'outer;
                    }
                    curr_workflow = workflows
                        .iter()
                        .find(|x| x.workflow == rule.send_to)
                        .unwrap();
                    continue 'outer;
                }
            }
            // if we didnt match any rules
            if curr_workflow.end == "A" || curr_workflow.end == "R" {
                match curr_workflow.end {
                    "A" => a += rating.iter().map(|x| x.1).sum::<u32>(),
                    "R" => (),
                    _ => panic!(),
                };
                break 'outer;
            }
            curr_workflow = workflows
                .iter()
                .find(|x| x.workflow == curr_workflow.end)
                .unwrap();
        }
    }
    a.to_string()
}
fn parse_input(input: &str) -> (Vec<Tape>, Vec<(String, u32)>) {
    let mut in_ratings = false;
    let mut workflows = Vec::new();
    let mut ratings = Vec::new();
    for line in lines(input) {
        if line.is_empty() {
            in_ratings = true;
            continue;
        }
        if in_ratings {
            let l = line.replace(&['{', '}'][..], "");
            let pairs: Vec<&str> = l.split(',').collect();
            for pair in pairs {
                let sp: Vec<&str> = pair.split('=').collect();
                let part = sp[0].to_string();
                let val = sp[1].parse::<u32>().unwrap();
                ratings.push((part.clone(), val));
            }
            continue;
        }
        let line: Vec<&str> = line.split('{').collect();
        let curr_workflow = line[0];
        let rules_s: Vec<&str> = line[1].split(',').collect();
        let end = rules_s.last().unwrap();
        let end = &end[0..end.len() - 1];
        let mut rules = Vec::new();
        for rule in rules_s.iter().take(rules_s.len() - 1) {
            let mut lt = false;
            let sp: Vec<&str> = rule.split(':').collect();
            let send_to = sp[1];
            let part = &sp[0][0..1];
            let amt = &sp[0][2..].parse::<u32>().unwrap();
            if rule.contains('<') {
                lt = true;
            }
            rules.push(Rule {
                part,
                lt,
                send_to,
                amt: *amt,
            });
        }
        let tape = Tape {
            workflow: curr_workflow,
            rules,
            end,
        };
        workflows.push(tape);
    }
    (workflows, ratings)
}
fn lines(input: &str) -> Vec<&str> {
    input.split('\n').collect()
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
    eprintln!("{:?}", start.elapsed());
}

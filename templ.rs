use std::env;

fn part2(input: &str) -> &str {
    todo!();
}
fn part1(input: &str) -> &str {
    todo!();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<u32>().unwrap();
    let mut input = String::new();
    let stdin = std::io::stdin();
    loop {
        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => continue,
            _ => continue,
        }
    }
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
            println!("not valid input madge");
        }
    }
}

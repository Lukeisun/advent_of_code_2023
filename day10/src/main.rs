use colored::*;
use std::{
    collections::{HashMap, VecDeque},
    env,
    io::{self, Read},
    process::exit,
};
#[derive(Debug, Clone, PartialEq)]
enum Dir {
    N,
    E,
    S,
    W,
}
#[derive(Debug, Clone)]
struct Pipe {
    dirs: Vec<Dir>,
    symbol: String,
    dist: u32,
    idx: (usize, usize),
}
// my general approach for testing if a move is valid
// is as follows
// -----
// if we want to go to a pipe from a certain direction,
// we check if the pipe's dir array has the opposite of that direction.
// if so we remove that direction from the Pipe's dir array.
// eg L has dirs [North, East]
//  |
//  L
//  if we move from L to | here, we check if | has the south direction
//  in its dir array. It does, so the south direction is popped and we can
//  only move north from itself. The same thing was done for the L, and whatever
//  came before it.
//
//  For the dir parameter, this is the direction that the previous pipe is coming from.
//  So when we go from L to |, dir would be north as thats the move we take to get to it.
//  ----
//  Maybe there is a better way to do this, but this is what came to mind.
//  As I was having
impl Pipe {
    fn can_move(&self, dir: &Dir) -> Option<Dir> {
        match dir {
            Dir::N => {
                if self.dirs.contains(&Dir::S) {
                    Some(Dir::S)
                } else {
                    None
                }
            }
            Dir::S => {
                if self.dirs.contains(&Dir::N) {
                    Some(Dir::N)
                } else {
                    None
                }
            }
            Dir::E => {
                if self.dirs.contains(&Dir::W) {
                    Some(Dir::W)
                } else {
                    None
                }
            }
            Dir::W => {
                if self.dirs.contains(&Dir::E) {
                    Some(Dir::E)
                } else {
                    None
                }
            }
        }
    }
}
fn input_to_game(input: &str) -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut game = Vec::new();
    let mut s_idx = (0, 0);
    for (i, line) in lines(input).into_iter().enumerate() {
        let translated: Vec<Pipe> = line
            .chars()
            .enumerate()
            .map(|x| match x.1 {
                '|' => Pipe {
                    dirs: vec![Dir::N, Dir::S],
                    symbol: x.1.to_string(),
                    dist: 0,
                    idx: (i, x.0),
                },
                '-' => Pipe {
                    dirs: vec![Dir::E, Dir::W],
                    symbol: x.1.to_string(),
                    dist: 0,
                    idx: (i, x.0),
                },
                'L' => Pipe {
                    dirs: vec![Dir::N, Dir::E],
                    symbol: x.1.to_string(),
                    dist: 0,
                    idx: (i, x.0),
                },
                'J' => Pipe {
                    dirs: vec![Dir::N, Dir::W],
                    symbol: x.1.to_string(),
                    dist: 0,
                    idx: (i, x.0),
                },
                '7' => Pipe {
                    dirs: vec![Dir::S, Dir::W],
                    symbol: x.1.to_string(),
                    dist: 0,
                    idx: (i, x.0),
                },
                'F' => Pipe {
                    dirs: vec![Dir::S, Dir::E],
                    symbol: x.1.to_string(),
                    dist: 0,
                    idx: (i, x.0),
                },
                '.' => Pipe {
                    dirs: vec![],
                    symbol: x.1.to_string(),
                    dist: 0,
                    idx: (i, x.0),
                },
                'S' => {
                    s_idx = (i, x.0);
                    Pipe {
                        dirs: vec![Dir::N, Dir::S, Dir::W, Dir::E],
                        symbol: x.1.to_string(),
                        dist: 0,
                        idx: (i, x.0),
                    }
                }
                _ => panic!("invalid"),
            })
            .collect();
        game.push(translated);
    }
    (game, s_idx)
}
fn part2(input: &str) -> String {
    let (mut game, s_idx) = input_to_game(input);
    let game_size = (game.len(), game[0].len());
    let mut visited = HashMap::new();
    let visitor = game[s_idx.0][s_idx.1].clone();
    bfs(&game, &game_size, &mut visited, visitor);
    // for row in game.clone() {
    //     let row: Vec<String> = row.into_iter().map(|x| x.symbol).collect();
    //     println!("\t{:?}", row.join(" "));
    // }
    // for (k, v) in &visited {
    //     game[k.0][k.1].symbol = 'V'.to_string();
    // }
    let mut candidates = Vec::new();
    for (i, row) in game.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if !visited.contains_key(&(i, j)) {
                col.symbol = '.'.to_string();
                candidates.push((i, j));
            }
        }
    }
    let mut num_i = 0;
    for candidate in candidates {
        let mut intersections = 0;
        let x = candidate.0 + 0;
        let mut y = candidate.1 + 1;
        while !(y >= game_size.1) {
            let mut symbol = &game[x][y].symbol;
            if symbol == "|" {
                intersections += 1;
            }
            if symbol == "F" {
                y += 1;
                symbol = &game[x][y].symbol;
                while symbol == "-" && !(y >= game_size.1) {
                    y += 1;
                    symbol = &game[x][y].symbol;
                }
                if symbol == "7" {
                    intersections += 2;
                } else if symbol == "J" {
                    intersections += 1;
                }
            } else if symbol == "L" {
                y += 1;
                symbol = &game[x][y].symbol;
                while symbol == "-" {
                    y += 1;
                    symbol = &game[x][y].symbol;
                }
                if symbol == "J" {
                    intersections += 2;
                } else if symbol == "7" {
                    intersections += 1;
                }
            }
            y += 1;
        }
        if intersections % 2 == 0 {
            game[candidate.0][candidate.1].symbol = ".".to_string();
        } else {
            game[candidate.0][candidate.1].symbol = "I".to_string();
            num_i += 1;
        }
    }

    // visualize
    let mut visual = String::new();
    for row in game.clone() {
        let row: Vec<String> = row
            .into_iter()
            .map(|x| match x.symbol.as_str() {
                "I" => x.symbol.blue().bold().to_string(),
                "|" => "┃".truecolor(100, 100, 100).to_string(),
                "-" => "━".truecolor(100, 100, 100).to_string(),
                "L" => "┗".truecolor(100, 100, 100).to_string(),
                "J" => "┛".truecolor(100, 100, 100).to_string(),
                "7" => "┓".truecolor(100, 100, 100).to_string(),
                "F" => "┏".truecolor(100, 100, 100).to_string(),
                _ => ".".black().to_string(),
            })
            .collect();
        visual.push_str(format!("{}\n", row.join("")).as_str());
    }
    println!("{}", visual);
    num_i.to_string()
}
fn part1(input: &str) -> String {
    let (mut game, s_idx) = input_to_game(input);
    let game_size = (game.len(), game[0].len());
    let mut visited = HashMap::new();
    let visitor = game[s_idx.0][s_idx.1].clone();
    bfs(&game, &game_size, &mut visited, visitor);
    // dfs(&game, &game_size, &mut visited, visitor, 0);
    for (k, v) in &visited {
        game[k.0][k.1].symbol = v.to_string();
    }
    // for row in game.clone() {
    //     let row: Vec<String> = row.into_iter().map(|x| x.symbol).collect();
    //     println!("\t{:?}", row);
    // }
    // for row in &game {
    //     let row: Vec<u32> = row.into_iter().map(|x| x.dist).collect();
    //     println!("\t{:?}", row);
    // }
    // println!("{:?}", visited);
    visited.into_values().max().unwrap().to_string()
}
fn bfs(
    game: &Vec<Vec<Pipe>>,
    game_size: &(usize, usize),
    visited: &mut HashMap<(usize, usize), u32>,
    visitor: Pipe,
) {
    let mut q = VecDeque::new();
    q.push_front(visitor);
    while !q.is_empty() {
        let visitor = q.pop_front().unwrap();
        visited.insert(visitor.idx, visitor.dist);
        for dir in &visitor.dirs {
            let (dx, dy) = match dir {
                Dir::N => (-1, 0),
                Dir::S => (1, 0),
                Dir::W => (0, -1),
                Dir::E => (0, 1),
            };
            let x = (visitor.idx.0 as i32 + dx) as usize;
            let y = (visitor.idx.1 as i32 + dy) as usize;
            if x >= game_size.0 || y >= game_size.1 || visited.contains_key(&(x, y)) {
                continue;
            }
            let mut potential_visitor = game[x][y].clone();
            let pred = potential_visitor.can_move(&dir);
            if let Some(dir) = pred {
                let dir_idx = potential_visitor
                    .dirs
                    .iter()
                    .position(|x| *x == dir)
                    .unwrap();
                potential_visitor.dirs.swap_remove(dir_idx);
                potential_visitor.dist += visitor.dist + 1;
                q.push_back(potential_visitor.clone());
            }
        }
    }
}
fn dfs(
    game: &Vec<Vec<Pipe>>,
    game_size: &(usize, usize),
    visited: &mut HashMap<(usize, usize), u32>,
    visitor: &Pipe,
    dist: u32,
) -> () {
    visited.insert(visitor.idx, dist);
    for dir in &visitor.dirs {
        let (dx, dy) = match dir {
            Dir::N => (-1, 0),
            Dir::S => (1, 0),
            Dir::W => (0, -1),
            Dir::E => (0, 1),
        };
        let x = (visitor.idx.0 as i32 + dx) as usize;
        let y = (visitor.idx.1 as i32 + dy) as usize;
        if x >= game_size.0 || y >= game_size.1 || visited.contains_key(&(x, y)) {
            continue;
        }
        let mut potential_visitor = game[x][y].clone();
        let pred = potential_visitor.can_move(&dir);
        if let Some(dir) = pred {
            let dir_idx = potential_visitor
                .dirs
                .iter()
                .position(|x| *x == dir)
                .unwrap();
            potential_visitor.dirs.swap_remove(dir_idx);
            potential_visitor.dist += 1;
            dfs(game, game_size, visited, &potential_visitor, dist + 1);
        }
    }
    ()
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

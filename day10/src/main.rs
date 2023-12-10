use std::{
    collections::{HashMap, VecDeque},
    env,
    io::{self, Read},
    process::exit,
};
#[derive(Debug, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
}
#[derive(Debug, Clone)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Starting,
    Num(usize),
}
impl Pipe {
    fn is_north(&self) -> bool {
        match self {
            Pipe::NW => true,
            Pipe::NE => true,
            Pipe::NS => true,
            _ => false,
        }
    }
    fn is_south(&self) -> bool {
        match self {
            Pipe::SW => true,
            Pipe::SE => true,
            Pipe::NS => true,
            _ => false,
        }
    }
    fn is_east(&self) -> bool {
        match self {
            Pipe::EW => true,
            Pipe::SE => true,
            Pipe::NE => true,
            _ => false,
        }
    }
    fn is_west(&self) -> bool {
        match self {
            Pipe::EW => true,
            Pipe::SW => true,
            Pipe::NW => true,
            _ => false,
        }
    }
    fn valid(&self, other: &Pipe, from: &Dir) -> bool {
        if let Pipe::Ground = other {
            return false;
        }
        if let Pipe::Starting = self {
            return true;
        }
        match self {
            Pipe::NW => other.is_east() || other.is_south(),
            Pipe::SW => other.is_east() || other.is_north(),
            Pipe::NS => other.is_north() || other.is_south(),
            Pipe::EW => other.is_east() || other.is_west(),
            Pipe::NE => other.is_west() || other.is_south(),
            Pipe::SE => other.is_west() || other.is_north(),
            _ => false,
        }
        // match other {
        //     Pipe::NW => self.is_east() || self.is_south(),
        //     Pipe::SW => self.is_east() || self.is_north(),
        //     Pipe::NS => match from {
        //         Dir::N => self.is_north(),
        //         Dir::S => self.is_south(),
        //         _ => false,
        //     },
        //     Pipe::EW => self.is_east() || self.is_west(),
        //     Pipe::NE => self.is_west() || self.is_south(),
        //     Pipe::SE => self.is_west() || self.is_north(),
        //     _ => false,
        // }
    }
}
fn part2(input: &str) -> String {
    todo!();
}
fn part1(input: &str) -> String {
    let mut game: Vec<Vec<Pipe>> = Vec::new();
    let mut s_idx = (0, 0);
    for (i, line) in lines(input).into_iter().enumerate() {
        let translated: Vec<Pipe> = line
            .chars()
            .enumerate()
            .map(|x| match x.1 {
                '|' => Pipe::NS,
                '-' => Pipe::EW,
                'L' => Pipe::NE,
                'J' => Pipe::NW,
                '7' => Pipe::SW,
                'F' => Pipe::SE,
                '.' => Pipe::Ground,
                'S' => {
                    s_idx = (i, x.0);
                    Pipe::Starting
                }
                _ => panic!("invalid"),
            })
            .collect();
        game.push(translated);
    }
    for r in game.clone() {
        println!("row {:?}", r);
    }
    //            N         S       E       W
    let dirs = [
        (1, 0, Dir::S),
        (-1, 0, Dir::N),
        (0, 1, Dir::E),
        (0, -1, Dir::W),
    ];
    // let dirs = [Dir::N(1, 0), Dir::S(-1, 0), Dir::E(0, 1), Dir::W(0, -1)];
    let row_len = game.len();
    let col_len = game[0].len();
    let mut connected_idxs = Vec::new();
    let mut q = VecDeque::new();
    let mut visited = HashMap::new();
    let x = (s_idx.0, s_idx.1, 0);
    q.push_front(x);
    connected_idxs.push(x);
    while !q.is_empty() {
        let cur_idx = q.pop_front().unwrap();
        visited.insert((cur_idx.0, cur_idx.1), ());
        for dir in &dirs {
            let dx = dir.0;
            let dy = dir.1;
            let x = (cur_idx.0 as i32 + dx) as usize;
            let y = (cur_idx.1 as i32 + dy) as usize;
            if x >= row_len || y >= col_len || visited.contains_key(&(x, y)) {
                continue;
            }
            let from_elem = &game[cur_idx.0][cur_idx.1];
            let to_elem = &game[x][y];
            if x == 1 && y == 3 {
                println!(
                    "{x}, {y} - {} from_elem {:?} to_elem {:?} dirs {:?}",
                    from_elem.valid(to_elem, &dir.2),
                    from_elem,
                    to_elem,
                    &dir.2
                );
            }
            let dist = cur_idx.2;
            if from_elem.valid(to_elem, &dir.2) {
                connected_idxs.push((x, y, dist + 1));
                q.push_back((x, y, dist + 1));
            }
        }
    }
    println!("{:?}", connected_idxs);
    for idx in connected_idxs {
        // println!("{:?} {:?}", idx, game[idx.0][idx.1]);
        game[idx.0][idx.1] = Pipe::Num(idx.2);
    }
    for r in game.clone() {
        println!("row {:?}", r);
    }
    todo!();
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

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    env,
    io::{self, Read},
    process::exit,
    usize,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    idx: (i32, i32),
    dir: Dir,
    amount_right: usize,
    acc: i32,
}
impl Node {
    fn add(&self, dir: (i32, i32)) -> (i32, i32) {
        (self.idx.0 + dir.0, self.idx.1 + dir.1)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.acc.cmp(&other.acc)
        // other.acc.cmp(&self.acc)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    UP = 0,    // (-1 ,0)
    DOWN = 1,  // (1,0)
    LEFT = 2,  // (0, -1)
    RIGHT = 3, // (0, 1)
}
impl Dir {
    fn rotate(&self) -> Vec<Dir> {
        match self {
            Dir::RIGHT | Dir::LEFT => vec![Dir::UP, Dir::DOWN],
            Dir::UP | Dir::DOWN => vec![Dir::RIGHT, Dir::LEFT],
        }
    }
    fn get_dir(&self) -> (i32, i32) {
        match &self {
            Dir::UP => (-1, 0),
            Dir::DOWN => (1, 0),
            Dir::LEFT => (0, -1),
            Dir::RIGHT => (0, 1),
        }
    }
}
fn part2(input: &str) -> String {
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }
    let loss = dijkstra(&board, 4, 10);
    loss.to_string()
}
fn part1(input: &str) -> String {
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }
    let loss = dijkstra(&board, 0, 3);
    loss.to_string()
}
fn in_range(idx: (i32, i32), row_len: i32, col_len: i32) -> bool {
    let row = idx.0;
    let col = idx.1;
    return 0 <= row && row < row_len && 0 <= col && col < col_len;
}
fn dijkstra(board: &Vec<Vec<u32>>, min: usize, max: usize) -> u32 {
    let mut q = BinaryHeap::new();
    let row_len = board.len() as i32;
    let col_len = board[0].len() as i32;
    let target = (row_len - 1, col_len - 1);
    let mut visited: HashMap<((i32, i32), Dir, usize), i32> = HashMap::new();
    let node_d = Node {
        idx: (0, 0),
        dir: Dir::DOWN,
        amount_right: 0,
        acc: 0,
    };
    q.push(node_d);
    let node = Node {
        idx: (0, 0),
        dir: Dir::RIGHT,
        amount_right: 0,
        acc: 0,
    };
    q.push(node);
    while let Some(node) = q.pop() {
        let k = (node.idx, node.dir.clone(), node.amount_right);
        let v = visited.get(&k).unwrap_or(&i32::MAX);
        let acc = -1 * node.acc;
        if node.idx == target && node.amount_right >= min {
            return acc as u32;
        }
        if acc > *v {
            continue;
        }
        let mut neighbors = Vec::new();
        let tup_dir = node.dir.get_dir();
        let new_idx = node.add(tup_dir);
        if in_range(new_idx, row_len, col_len) && node.amount_right < max {
            neighbors.push((new_idx, node.amount_right + 1, node.dir.clone()));
        }
        if node.amount_right >= min {
            let rotated_dirs = node.dir.rotate();
            for rotated_dir in rotated_dirs {
                let tup_dir = rotated_dir.get_dir();
                let new_idx = node.add(tup_dir);
                if in_range(new_idx, row_len, col_len) {
                    neighbors.push((new_idx, 1, rotated_dir));
                }
            }
        }
        for neighbor in neighbors {
            let n_idx = neighbor.0;
            let n_val = board[n_idx.0 as usize][n_idx.1 as usize] as i32;
            let n_right = neighbor.1;
            let n_dir = neighbor.2;
            let next_cost = acc + n_val;
            let next_node = Node {
                idx: n_idx,
                dir: n_dir,
                amount_right: n_right,
                acc: -next_cost,
            };
            let k = (next_node.idx, next_node.dir.clone(), next_node.amount_right);
            let v = visited.get(&k).unwrap_or(&i32::MAX);
            if next_cost < *v {
                visited.insert(k, next_cost);
                q.push(next_node);
            }
        }
    }
    0
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
    eprintln!("{:?}", start.elapsed());
}

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    env,
    io::{self, Read},
    process::exit,
    rc::Rc,
    u128::MAX,
    usize,
};

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
    todo!();
}
fn part1(input: &str) -> String {
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines(input) {
        board.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }
    let loss = go(&board, (0, 0), Dir::RIGHT);
    loss.to_string()
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<'a> {
    idx: (i32, i32),
    dir: &'a Dir,
    amount_right: usize,
    acc: i32,
}
impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.acc.cmp(&other.acc)
        // other.acc.cmp(&self.acc)
    }
}
impl PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn dijkstra(board: &Vec<Vec<u32>>, node: &Node) -> u32 {
    let mut q = BinaryHeap::new();
    let row_len = board.len() as i32;
    let col_len = board[0].len() as i32;
    let target = (row_len - 1, col_len - 1);
    let mut visited = HashMap::new();
    let dirs = vec![Dir::UP, Dir::DOWN, Dir::LEFT, Dir::RIGHT];
    let node_d = Node {
        idx: node.idx,
        dir: &Dir::DOWN,
        amount_right: 0,
        acc: 0,
    };
    let node_r = Node {
        idx: node.idx,
        dir: &Dir::RIGHT,
        amount_right: 0,
        acc: 0,
    };
    q.push(node_d);
    q.push(node_r);
    // q.push(node.clone());
    while let Some(Node {
        idx,
        dir,
        amount_right,
        acc,
    }) = q.pop()
    {
        let acc = -1 * acc;
        let k = (idx, dir, amount_right);
        if acc > *visited.get(&k).unwrap_or(&i32::MAX) {
            continue;
        }
        visited.insert(k, acc);
        if idx == target {
            return acc as u32;
        }
        let (dx, dy) = dir.get_dir();
        let mut r = idx.0;
        let mut c = idx.1;
        let mut cost = acc;
        for i in 0..2 {
            r += dx;
            c += dy;
            if in_range(r, c, row_len, col_len) && amount_right == 0 {
                let next_cost = board[r as usize][c as usize] as i32;
                cost += next_cost;
                let potential = Node {
                    idx: (r, c),
                    dir,
                    amount_right: i + 1,
                    acc: -(cost),
                };
                q.push(potential);
            } else {
                break;
            }
        }
        let rotated_dirs = dir.rotate();
        for dir in rotated_dirs {
            let (dx, dy) = dir.get_dir();
            // let
            let (r, c) = (idx.0 + dx, idx.1 + dy);
            if in_range(r, c, row_len, col_len) {
                let next_cost = board[r as usize][c as usize] as i32;
                let cost = acc + next_cost;
                let dir = &dirs[dirs.iter().position(|x| *x == dir).unwrap()];
                q.push(Node {
                    idx: (r, c),
                    dir,
                    amount_right: 0,
                    acc: -cost,
                });
            }
        }
    }
    0
}
// do dijkstra
fn go(board: &Vec<Vec<u32>>, idx: (i32, i32), s_dir: Dir) -> u32 {
    // let mut visited = HashMap::new();
    let node_d = Node {
        idx,
        dir: &Dir::DOWN,
        amount_right: 0,
        acc: 0,
    };
    let node_r = Node {
        idx,
        dir: &Dir::RIGHT,
        amount_right: 0,
        acc: 0,
    };
    dijkstra(board, &node_d)
    // dijkstra(board, &node_r).min(dijkstra(board, &node_d))
}
fn in_range(row: i32, col: i32, row_len: i32, col_len: i32) -> bool {
    return 0 <= row && row < row_len && 0 <= col && col < col_len;
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

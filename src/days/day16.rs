use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use Cardinal::*;

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| l
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                'S' => { start = Some((x, y)); '.' },
                'E' => { end = Some((x, y)); '.' },
                _ => c,
            })
            .collect()
        ).collect();
    (grid, start.unwrap(), end.unwrap())
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Cardinal {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Cardinal {
    fn move_by(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        }
    }

    fn rotate_90(&self) -> [Self; 2] {
        match self {
            North | South => [East, West],
            East | West => [North, South],
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct NodeEntry((usize, usize), Cardinal, usize);

impl PartialOrd<Self> for NodeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}

fn search(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::new();
    dists.insert((start, East), 0);
    queue.push(NodeEntry(start, East, 0));

    while let Some(NodeEntry(pos, dir, dist)) = queue.pop() {
        if pos == end {
            return dist;
        }

        let np @ (nx, ny) = dir.move_by(pos);
        if grid[ny][nx] == '.' && dists.get(&(np, dir)).is_none_or(|&d| dist + 1 < d) {
            dists.insert((np, dir), dist + 1);
            queue.push(NodeEntry(np, dir, dist + 1));
        }

        for nd in dir.rotate_90() {
            if dists.get(&(pos, nd)).is_none_or(|&d| dist + 1000 < d) {
                dists.insert((pos, nd), dist + 1000);
                queue.push(NodeEntry(pos, nd, dist + 1000));
            }
        }
    }

    panic!("no paths found");
}

pub(crate) fn part1(input: String) {
    let (grid, start, end) = parse(&input);
    println!("{}", search(&grid, start, end));
}

fn all_shortest_paths(grid: &Vec<Vec<char>>, start: (usize, usize), dir: Cardinal, end: (usize, usize), score: usize, max_score: usize, mut so_far: HashSet<(usize, usize)>, global_memo: &mut HashMap<((usize, usize), Cardinal), usize>) -> Vec<HashSet<(usize, usize)>> {
    so_far.insert(start);
    if start == end {
        return vec![so_far];
    }

    let mut v = vec![];

    for nd in [North, South, East, West] {
        let np @ (nx, ny) = nd.move_by(start);
        if !so_far.contains(&np) && grid[ny][nx] == '.' {
            let new_score = if nd == dir {
                score + 1
            } else {
                score + 1001
            };

            if new_score <= max_score && global_memo.get(&(np, nd)).is_none_or(|&m| m >= new_score) {
                global_memo.insert((np, nd), new_score);
                v.append(&mut all_shortest_paths(grid, np, nd, end, new_score, max_score, so_far.clone(), global_memo));
            }
        }
    }

    v
}

pub(crate) fn part2(input: String) {
    let (grid, start, end) = parse(&input);
    let shortest = search(&grid, start, end);

    let mut points = HashSet::new();
    let mut global_memo = HashMap::new();
    let all_paths = all_shortest_paths(&grid, start, East, end, 0, shortest, HashSet::new(), &mut global_memo);
    for path in all_paths {
        points.extend(path);
    }
    println!("{}", points.len());
}

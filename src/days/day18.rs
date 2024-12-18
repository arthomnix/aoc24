use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const SIZE: usize = 70;
const BYTES: usize = 1024;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct NodeEntry {
    pos: (usize, usize),
    dist: usize,
}

impl NodeEntry {
    fn h(&self) -> usize {
        (SIZE - self.pos.0).max(SIZE - self.pos.1)
    }
}

impl PartialOrd for NodeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.dist + self.h();
        let d2 = other.dist + other.h();
        d2.cmp(&d1)
    }
}


fn search(corrupted: &HashSet<(usize, usize)>) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    queue.push(NodeEntry { pos: (0, 0), dist: 0});

    while let Some(NodeEntry { pos: (x, y), dist }) = queue.pop() {
        if (x, y) == (SIZE, SIZE) {
            return Some(dist);
        }

        visited.insert((x, y), dist);

        if x > 0 && !corrupted.contains(&(x - 1, y)) && visited.get(&(x - 1, y)).is_none_or(|&d| d > dist + 1) {
            queue.push(NodeEntry { pos: (x - 1, y), dist: dist + 1 });
        }
        if x < SIZE && !corrupted.contains(&(x + 1, y)) && visited.get(&(x + 1, y)).is_none_or(|&d| d > dist + 1) {
            queue.push(NodeEntry { pos: (x + 1, y), dist: dist + 1 });
        }
        if y > 0 && !corrupted.contains(&(x, y - 1)) && visited.get(&(x, y - 1)).is_none_or(|&d| d > dist + 1) {
            queue.push(NodeEntry { pos: (x, y - 1), dist: dist + 1 });
        }
        if y < SIZE && !corrupted.contains(&(x, y + 1)) && visited.get(&(x, y + 1)).is_none_or(|&d| d > dist + 1) {
            queue.push(NodeEntry { pos: (x, y + 1), dist: dist + 1 });
        }
    }

    None
}

fn parse(input: &str, n: usize) -> HashSet<(usize, usize)> {
    input
        .lines()
        .take(n)
        .map(|l| {
            let (x, y) = l.trim().split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub(crate) fn part1(input: String) {
    let corrupted = parse(&input, BYTES);
    println!("{}", search(&corrupted).unwrap());
}

pub(crate) fn part2(input: String) {
    let mut lb = BYTES;
    let mut ub = input.lines().count() - 1;
    loop {
        if lb == ub - 1 {
            println!("{}", input.lines().nth(lb).unwrap());
            break;
        }

        let midpoint = (ub + lb) / 2;
        let corrupted = parse(&input, midpoint);
        if search(&corrupted).is_some() {
            lb = midpoint;
        } else {
            ub = midpoint;
        }
    }
}

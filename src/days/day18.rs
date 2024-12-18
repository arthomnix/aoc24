use std::collections::{HashMap, HashSet, VecDeque};

const SIZE: usize = 70;
const BYTES: usize = 1024;

fn search(corrupted: &HashSet<(usize, usize)>) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(((0, 0), 0));

    while let Some(((x, y), dist)) = queue.pop_front() {
        if (x, y) == (SIZE, SIZE) {
            return Some(dist);
        }

        if x > 0 && !corrupted.contains(&(x - 1, y)) && visited.get(&(x - 1, y)).is_none_or(|&d| d > dist + 1) {
            queue.push_back(((x - 1, y), dist + 1));
            visited.insert((x - 1, y), dist);
        }
        if x < SIZE && !corrupted.contains(&(x + 1, y)) && visited.get(&(x + 1, y)).is_none_or(|&d| d > dist + 1) {
            queue.push_back(((x + 1, y), dist + 1));
            visited.insert((x + 1, y), dist);
        }
        if y > 0 && !corrupted.contains(&(x, y - 1)) && visited.get(&(x, y - 1)).is_none_or(|&d| d > dist + 1) {
            queue.push_back(((x, y - 1), dist + 1));
            visited.insert((x, y - 1), dist);
        }
        if y < SIZE && !corrupted.contains(&(x, y + 1)) && visited.get(&(x, y + 1)).is_none_or(|&d| d > dist + 1) {
            queue.push_back(((x, y + 1), dist + 1));
            visited.insert((x, y + 1), dist);
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

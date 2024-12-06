use std::collections::HashSet;
use Direction::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn90(&self) -> Self {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

fn parse(input: String) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, l)| l
            .iter()
            .enumerate()
            .find(|(y, &c)| c == '^')
            .map(|(x, c)| (x, y)))
        .unwrap();
    (grid, pos)
}

fn get_visited(grid: &Vec<Vec<char>>, mut pos: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
    let mut dir = Up;
    let width = grid[0].len();
    let height = grid.len();
    let mut visited = HashSet::from([(pos, Up)]);

    loop {
        let next_pos = match dir {
            Up => (pos.0, pos.1 - 1),
            Down => (pos.0, pos.1 + 1),
            Left => (pos.0 - 1, pos.1),
            Right => (pos.0 + 1, pos.1),
        };

        if grid[next_pos.1][next_pos.0] == '#' {
            dir = dir.turn90();
        } else {
            pos = next_pos;
        }

        if !visited.insert((pos, dir)) {
            return None; // there is a loop
        };

        if (pos.0 == width - 1 && dir == Right)
            || (pos.0 == 0 && dir == Left)
            || (pos.1 == height - 1 && dir == Down)
            || (pos.1 == 0 && dir == Up)
        {
            break;
        }
    }

    Some(visited.into_iter().map(|(pos, _)| pos).collect())
}

pub(crate) fn part1(input: String) {
    let (grid, pos) = parse(input);
    let visited = get_visited(&grid, pos).unwrap();
    println!("{}", visited.len());
}

pub(crate) fn part2(input: String) {
    // janky brute force approach
    let (grid, pos) = parse(input);
    let visited = get_visited(&grid, pos).unwrap();
    let width = grid[0].len();
    let height = grid.len();

    let count = visited.into_iter()
        .filter(|&cand| {
            if cand == pos {
                false // don't put an obstacle at the start
            } else {
                let mut new_grid = grid.clone();
                new_grid[cand.1][cand.0] = '#';
                get_visited(&new_grid, pos).is_none()
            }
        })
        .count();

    println!("{count}");
}

use colored::Colorize;
use crate::days::day15::Dir::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn move_by(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }

    fn move_by_rev(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Up => (x, y + 1),
            Down => (x, y - 1),
            Left => (x + 1, y),
            Right => (x - 1, y),
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Up),
            'v' => Ok(Down),
            '<' => Ok(Left),
            '>' => Ok(Right),
            _ => Err(()),
        }
    }
}

fn parse<'a>(input: &'a str) -> (Vec<Vec<char>>, (usize, usize), impl Iterator<Item = Dir> + use<'a>) {
    let (g, v) = input.split_once("\n\n").unwrap();
    let mut start = None;
    let grid = g
        .lines()
        .enumerate()
        .map(|(y, l)| l
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                '@' => {
                    start = Some((x, y));
                    '.'
                },
                c => c,
            })
            .collect()
        )
        .collect();
    let moves = v
        .chars()
        .filter_map(|c| c.try_into().ok());

    (grid, start.unwrap(), moves)
}

fn try_move(grid: &mut Vec<Vec<char>>, dir: Dir, pos: (usize, usize)) -> (usize, usize) {
    let (nx, ny) = dir.move_by(pos);
    if grid[ny][nx] == '.' {
        return (nx, ny);
    }

    let mut p = (nx, ny);
    loop {
        let (px, py) = p;
        let c = grid[py][px];
        match c {
            '#' => break pos,
            '.' => {
                grid[py][px] = 'O';
                grid[ny][nx] = '.';
                break (nx, ny);
            },
            _ => {},
        }
        p = dir.move_by(p);
    }
}

fn gps(grid: &Vec<Vec<char>>) -> usize {
    grid
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l
            .iter()
            .enumerate()
            .filter_map(move |(x, c)| match *c {
                'O' | '[' => Some(x + 100 * y),
                _ => None,
            })
        )
        .sum()
}

fn visualise(grid: &Vec<Vec<char>>, (rx, ry): (usize, usize)) {
    for (y, l) in grid.iter().enumerate() {
        for (x, &c) in l.iter().enumerate() {
            if (x, y) == (rx, ry) {
                print!("{}", "@".bright_white());
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}

pub(crate) fn part1(input: String) {
    let (mut grid, mut pos, moves) = parse(&input);

    for dir in moves {
        pos = try_move(&mut grid, dir, pos);
    }

    visualise(&grid, pos);
    println!("{}", gps(&grid));
}

fn try_move_box_vertical(grid: &mut Vec<Vec<char>>, dir: Dir, pos: (usize, usize), do_move: bool) -> bool {
    let (left_pos, right_pos) = match grid[pos.1][pos.0] {
        '[' => (pos, (pos.0 + 1, pos.1)),
        ']' => ((pos.0 - 1, pos.1), pos),
        _ => panic!("invalid box"),
    };

    let (lnx, lny) = dir.move_by(left_pos);
    let (rnx, rny) = dir.move_by(right_pos);

    if grid[lny][lnx] == '#' || grid[rny][rnx] == '#' {
        return false;
    }

    let mut can_move = true;
    let mut try_right = true;

    match grid[lny][lnx] {
        '[' => {
            assert_eq!(grid[rny][rnx], ']');
            try_right = false;
            can_move &= try_move_box_vertical(grid, dir, (lnx, lny), do_move);
        },
        ']' => can_move &= try_move_box_vertical(grid, dir, (lnx, lny), do_move),
        _ => {},
    }

    if try_right && grid[rny][rnx] == '[' {
        can_move &= try_move_box_vertical(grid, dir, (rnx, rny), do_move);
    }

    if do_move && can_move {
        grid[lny][lnx] = '[';
        grid[rny][rnx] = ']';
        grid[left_pos.1][left_pos.0] = '.';
        grid[right_pos.1][right_pos.0] = '.';
    }


    can_move
}

fn try_move_box(grid: &mut Vec<Vec<char>>, dir: Dir, pos: (usize, usize)) -> bool {
    assert!(matches!(grid[pos.1][pos.0], '[' | ']'));
    match dir {
        Left | Right => {
            let mut p = pos;
            loop {
                let (px, py) = p;
                let c = grid[py][px];
                match c {
                    '#' => return false,
                    '.' => break,
                    _ => {},
                }
                p = dir.move_by(p);
            };

            let mut f = dir == Left;
            while p != pos {
                let (px, py) = p;
                grid[py][px] = if f { '[' } else { ']' };
                f = !f;
                p = dir.move_by_rev(p);
            }

            grid[pos.1][pos.0] = '.';

            true
        },
        Up | Down => {
            if try_move_box_vertical(grid, dir, pos, false) {
                try_move_box_vertical(grid, dir, pos, true)
            } else {
                false
            }
        },
    }
}

fn try_move_p2(grid: &mut Vec<Vec<char>>, dir: Dir, pos: (usize, usize)) -> (usize, usize) {
    let new @ (nx, ny) = dir.move_by(pos);

    match grid[ny][nx].clone() {
        '.' => new,
        '#' => pos,
        _ if try_move_box(grid, dir, new) => new,
        _ => pos,
    }
}

pub(crate) fn part2(input: String) {
    let input = input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    let (mut grid, mut pos, moves) = parse(&input);

    for dir in moves {
        pos = try_move_p2(&mut grid, dir, pos);
    }

    visualise(&grid, pos);
    println!("{}", gps(&grid));
}

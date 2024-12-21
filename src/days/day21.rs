use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use Move::*;

trait Position {
    fn position(&self) -> (i32, i32);

    fn default_position() -> (i32, i32);

    fn can_move_y_first(p: (i32, i32), np: (i32, i32)) -> bool;

    fn can_move_x_first(p: (i32, i32), np: (i32, i32)) -> bool;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Up => write!(f, "^"),
            Down => write!(f, "v"),
            Left => write!(f, "<"),
            Right => write!(f, ">"),
            Activate => write!(f, "A"),
        }
    }
}

impl Position for Move {
    fn position(&self) -> (i32, i32) {
        match self {
            Up => (1, 0),
            Activate => (2, 0),
            Left => (0, 1),
            Down => (1, 1),
            Right => (2, 1),
        }
    }

    fn default_position() -> (i32, i32) {
        Activate.position()
    }

    fn can_move_y_first((x, y): (i32, i32), (nx, ny): (i32, i32)) -> bool {
        !(x == 0 && ny == 0)
    }

    fn can_move_x_first((x, y): (i32, i32), (nx, ny): (i32, i32)) -> bool {
        !(y == 0 && nx == 0)
    }
}

impl Position for u8 {
    fn position(&self) -> (i32, i32) {
        match self {
            b'7' => (0, 0),
            b'8' => (1, 0),
            b'9' => (2, 0),
            b'4' => (0, 1),
            b'5' => (1, 1),
            b'6' => (2, 1),
            b'1' => (0, 2),
            b'2' => (1, 2),
            b'3' => (2, 2),
            b'0' => (1, 3),
            b'A' => (2, 3),
            _ => panic!("invalid key on numeric keypad!"),
        }
    }

    fn default_position() -> (i32, i32) {
        b'A'.position()
    }

    fn can_move_y_first((x, y): (i32, i32), (nx, ny): (i32, i32)) -> bool {
        !(x == 0 && ny == 3)
    }

    fn can_move_x_first((x, y): (i32, i32), (nx, ny): (i32, i32)) -> bool {
        !(y == 3 && nx == 0)
    }
}

fn find<T: Position>(code: &[T], start: Option<Move>) -> Vec<Move> {
    let mut moves = vec![];
    let (mut x, mut y) = start.map(|m| m.position()).unwrap_or_else(T::default_position);

    for m in code {
        let (nx, ny) = m.position();
        let (dx, dy) = (nx - x, ny - y);

        let mut x_moves = Vec::with_capacity(dx.abs() as usize);
        if dx < 0 {
            for _ in 0..dx.abs() {
                x_moves.push(Left);
            }
        } else {
            for _ in 0..dx {
                x_moves.push(Right);
            }
        }

        let mut y_moves = Vec::with_capacity(dy.abs() as usize);
        if dy < 0 {
            for _ in 0..dy.abs() {
                y_moves.push(Up);
            }
        } else {
            for _ in 0..dy {
                y_moves.push(Down);
            }
        }

        if (dx < 0 && T::can_move_x_first((x, y), (nx, ny))) || !T::can_move_y_first((x, y), (nx, ny)) {
            moves.append(&mut x_moves);
            moves.append(&mut y_moves);
        } else {
            moves.append(&mut y_moves);
            moves.append(&mut x_moves);
        }

        moves.push(Activate);
        (x, y) = (nx, ny);
    }

    moves
}

fn find_memo(code: Move, prev: Move, i: usize, memo: &mut HashMap<(Move, Move, usize), usize>) -> usize {
    if i == 0 {
        return 1;
    }

    if let Some(x) = memo.get(&(code, prev, i)) {
        return *x;
    }

    let mut v = find(&[code], Some(prev));
    let mut vn = vec![Activate];
    vn.append(&mut v);

    let res: usize = vn
        .windows(2)
        .map(|m| find_memo(m[1], m[0], i - 1, memo))
        .sum();

    memo.insert((code, prev, i), res);
    res
}

fn day(input: String, n: usize) {
    let mut memo = HashMap::new();
    let res = input
        .lines()
        .map(|s| {
            let mut v = find(s.as_bytes(), None);
            let mut vn = vec![Activate];
            vn.append(&mut v);

            let res = vn
                .windows(2)
                .map(|w| find_memo(w[1], w[0], n, &mut memo))
                .sum::<usize>();

            res * s[..3].parse::<usize>().unwrap()
        })
        .sum::<usize>();
    println!("{res}");
}

pub(crate) fn part1(input: String) {
    day(input, 2);
}

pub(crate) fn part2(input: String) {
    day(input, 25);
}

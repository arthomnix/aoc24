use std::collections::{HashMap, HashSet};

fn day(input: String, part2: bool) {
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;

    let mut antennae = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                antennae
                    .entry(c)
                    .and_modify(|e: &mut Vec<_>| e.push((x, y)))
                    .or_insert(vec![(x, y)]);
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_, points) in antennae {
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let (x1, y1) = points[i];
                let (x1, y1) = (x1 as isize, y1 as isize);

                let (x2, y2) = points[j];
                let (x2, y2) = (x2 as isize, y2 as isize);

                let (vx, vy) = (x2 - x1, y2 - y1);

                if !part2 {
                    let (ax1, ay1) = (x2 + vx, y2 + vy);
                    let (ax2, ay2) = (x1 - vx, y1 - vy);

                    if ax1 >= 0 && ay1 >= 0 && ax1 < width && ay1 < height {
                        antinodes.insert((ax1, ay1));
                    }
                    if ax2 >= 0 && ay2 >= 0 && ax2 < width && ay2 < height {
                        antinodes.insert((ax2, ay2));
                    }
                } else {
                    let (mut x, mut y) = (x2, y2);
                    while x >= 0 && y >= 0 && x < width && y < width {
                        antinodes.insert((x, y));
                        x += vx;
                        y += vy;
                    }

                    let (mut x, mut y) = (x1, y1);
                    while x >= 0 && y >= 0 && x < width && y < width {
                        antinodes.insert((x, y));
                        x -= vx;
                        y -= vy;
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

pub(crate) fn part1(input: String) {
    day(input, false);
}

pub(crate) fn part2(input: String) {
    day(input, true);
}

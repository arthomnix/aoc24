use colored::Colorize;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const MIDDLE_X: i64 = WIDTH / 2;
const MIDDLE_Y: i64 = HEIGHT / 2;

pub(crate) fn part1(input: String) {
    let (a, b, c, d) = input
        .lines()
        .map(|l| {
            let mut i = l.split(&['=', ',', ' ']).filter_map(|s| s.parse::<i64>().ok());
            let px = i.next().unwrap();
            let py = i.next().unwrap();
            let vx = i.next().unwrap();
            let vy = i.next().unwrap();
            match ((px + vx * 100).rem_euclid(WIDTH), (py + vy * 100).rem_euclid(HEIGHT)) {
                (x, y) if x < MIDDLE_X && y < MIDDLE_Y => (1, 0, 0, 0),
                (x, y) if x > MIDDLE_X && y < MIDDLE_Y => (0, 1, 0, 0),
                (x, y) if x < MIDDLE_X && y > MIDDLE_Y => (0, 0, 1, 0),
                (x, y) if x > MIDDLE_X && y > MIDDLE_Y => (0, 0, 0, 1),
                _ => (0, 0, 0, 0),
            }
        })
        .reduce(|(a, b, c, d), (e, f, g, h)| (a + e, b + f, c + g, d + h))
        .unwrap();
    println!("{}", a * b * c * d);
}

pub(crate) fn part2(input: String) {
    let mut robots: Vec<((i64, i64), (i64, i64))> = input.lines().map(|l| {
        let mut i = l.split(&['=', ',', ' ']).filter_map(|s| s.parse::<i64>().ok());
        ((i.next().unwrap(), i.next().unwrap()), (i.next().unwrap(), i.next().unwrap()))
    }).collect();

    let mut iv: Option<i64> = None;
    let mut ih: Option<i64> = None;
    let mut i = 0;
    'l: loop {
        let mut grid = [[0i64; WIDTH as usize]; HEIGHT as usize];
        for &((x, y), _) in robots.iter() {
            grid[y as usize][x as usize] += 1;
        }

        if let (Some(iv), Some(ih)) = (iv, ih) {
            if ((i - iv) % WIDTH) == 0 && ((i - ih) % HEIGHT) == 0 {
                for line in grid {
                    for num in line {
                        if num > 0 {
                            print!("{}", "#".green());
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!("i = {i}");
                break;
            }
        } else {
            const THRESHOLD: u64 = 10;

            if iv.is_none() {
                let mean_robots_per_col = robots.len() as i64 / WIDTH;
                for col in 0..WIDTH as usize {
                    let robots = (0..HEIGHT as usize).map(|y| grid[y][col]).sum::<i64>();
                    if robots.abs_diff(mean_robots_per_col) > THRESHOLD {
                        iv = Some(i);
                        continue 'l;
                    }
                }
            }

            if ih.is_none() {
                let mean_robots_per_row = robots.len() as i64 / HEIGHT;
                for row in 0..HEIGHT as usize {
                    let robots = (0..WIDTH as usize).map(|x| grid[row][x]).sum::<i64>();
                    if robots.abs_diff(mean_robots_per_row) > THRESHOLD {
                        ih = Some(i);
                        continue 'l;
                    }
                }
            }
        }

        for ((x, y), (vx, vy)) in robots.iter_mut() {
            *x = (*x + *vx).rem_euclid(WIDTH);
            *y = (*y + *vy).rem_euclid(HEIGHT);
        }

        i += 1;
    }
}

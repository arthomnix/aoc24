use std::collections::HashSet;

fn search(grid: &Vec<Vec<u8>>, start: (usize, usize), mut seen: HashSet<(usize, usize)>) -> (HashSet<(usize, usize)>, usize) {
    let (x, y) = start;
    if grid[y][x] == 9 {
        return (HashSet::from([(x, y)]), 1);
    }

    seen.insert((x, y));

    let mut endpoints = HashSet::new();
    let mut rating = 0;

    if x > 0 && grid[y][x - 1] == grid[y][x] + 1 && !seen.contains(&(x - 1, y)) {
        let (e, r) = search(grid, (x - 1, y), seen.clone());
        endpoints.extend(e);
        rating += r;
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] == grid[y][x] + 1 && !seen.contains(&(x + 1, y)) {
        let (e, r) = search(grid, (x + 1, y), seen.clone());
        endpoints.extend(e);
        rating += r;
    }
    if y > 0 && grid[y - 1][x] == grid[y][x] + 1 && !seen.contains(&(x, y - 1)) {
        let (e, r) = search(grid, (x, y - 1), seen.clone());
        endpoints.extend(e);
        rating += r;
    }
    if y < grid.len() - 1 && grid[y + 1][x] == grid[y][x] + 1 && !seen.contains(&(x, y + 1)) {
        let (e, r) = search(grid, (x, y + 1), seen.clone());
        endpoints.extend(e);
        rating += r;
    }

    (endpoints, rating)
}

fn day(input: String, part2: bool) {
    let grid = input.trim().lines().map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>()).collect::<Vec<_>>();
    let total_score = grid.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().filter_map(|(x, &c)| {
            if c == 0 {
                let (e, r) = search(&grid, (x, y), HashSet::new());
                if part2 {
                    Some(r)
                } else {
                    Some(e.len())
                }
            } else {
                None
            }
        }).sum::<usize>()
    }).sum::<usize>();

    println!("{total_score}");
}

pub(crate) fn part1(input: String) {
    day(input, false);
}

pub(crate) fn part2(input: String) {
    day(input, true);
}

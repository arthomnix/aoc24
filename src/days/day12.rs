use std::collections::HashSet;

fn region(grid: &Vec<Vec<char>>, start: (usize, usize), seen: &mut HashSet<(usize, usize)>) -> usize {
    let (x, y) = start;
    let c = grid[y][x];
    let width = grid[0].len();
    let height = grid.len();

    let mut perimeter = 0;
    seen.insert(start);

    let mut this_perimeter = 4;
    if x > 0 && grid[y][x - 1] == c {
        if !seen.contains(&(x - 1, y)) {
            perimeter += region(grid, (x - 1, y), seen);
        }
        this_perimeter -= 1;
    }
    if x < width - 1 && grid[y][x + 1] == c {
        if !seen.contains(&(x + 1, y)) {
            perimeter += region(grid, (x + 1, y), seen);
        }
        this_perimeter -= 1;
    }
    if y > 0 && grid[y - 1][x] == c {
        if !seen.contains(&(x, y - 1)) {
            perimeter += region(grid, (x, y - 1), seen);
        }
        this_perimeter -= 1;
    }
    if y < height - 1 && grid[y + 1][x] == c {
        if !seen.contains(&(x, y + 1)) {
            perimeter += region(grid, (x, y + 1), seen);
        }
        this_perimeter -= 1;
    }

    perimeter + this_perimeter
}

fn next_start(grid: &Vec<Vec<char>>, seen: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(y, l)| {
        l.iter().enumerate().find(|&(x, _)| !seen.contains(&(x, y))).map(|(x, _)| (x, y))
    })
}

pub(crate) fn part1(input: String) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut all_regions = HashSet::new();
    let mut count = 0;
    loop {
        let mut region_points = HashSet::new();
        if let Some(start) = next_start(&grid, &all_regions) {
            let perimeter = region(&grid, start, &mut region_points);
            count += perimeter * region_points.len();
            all_regions.extend(region_points);
        } else {
            break;
        }
    }

    println!("{count}");
}

pub(crate) fn part2(input: String) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut all_regions = HashSet::new();
    let mut count = 0;
    loop {
        let mut region_points = HashSet::new();
        if let Some(start) = next_start(&grid, &all_regions) {
            region(&grid, start, &mut region_points);

            let start_char = grid[start.1][start.0];
            let min_x = region_points.iter().map(|&(x, _)| x).min().unwrap();
            let max_x = region_points.iter().map(|&(x, _)| x).max().unwrap();
            let min_y = region_points.iter().map(|&(_, y)| y).min().unwrap();
            let max_y = region_points.iter().map(|&(_, y)| y).max().unwrap();

            let mut sides = 0;
            // scan horizontally
            for y in min_y..=max_y {
                let mut inside_above = true;
                let mut inside_below = true;
                for x in min_x..=max_x {
                    if region_points.contains(&(x, y)) {
                        if inside_above && ((y > 0 && grid[y - 1][x] != start_char) || y == 0) {
                            sides += 1;
                            inside_above = false;
                        }
                        if !inside_above && y > 0 && grid[y - 1][x] == start_char {
                            inside_above = true;
                        }

                        if inside_below && ((y < grid.len() - 1 && grid[y + 1][x] != start_char) || y == grid.len() - 1) {
                            sides += 1;
                            inside_below = false;
                        }
                        if !inside_below && y < grid.len() - 1 && grid[y + 1][x] == start_char {
                            inside_below = true;
                        }
                    } else {
                        inside_above = true;
                        inside_below = true;
                    }
                }
            }
            // scan vertically
            for x in min_x..=max_x {
                let mut inside_left = true;
                let mut inside_right = true;
                for y in min_y..=max_y {
                    if region_points.contains(&(x, y)) {
                        if inside_left && ((x > 0 && grid[y][x - 1] != start_char) || x == 0) {
                            sides += 1;
                            inside_left = false;
                        }
                        if !inside_left && x > 0 && grid[y][x - 1] == start_char {
                            inside_left = true;
                        }

                        if inside_right && ((x < grid[0].len() - 1 && grid[y][x + 1] != start_char) || x == grid[0].len() - 1) {
                            sides += 1;
                            inside_right = false;
                        }
                        if !inside_right && x < grid[0].len() - 1 && grid[y][x + 1] == start_char {
                            inside_right = true;
                        }
                    } else {
                        inside_left = true;
                        inside_right = true;
                    }
                }
            }

            count += sides * region_points.len();

            all_regions.extend(region_points);
        } else {
            break;
        }
    }

    println!("{count}");
}

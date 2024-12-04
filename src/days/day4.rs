pub(crate) fn part1(input: String) {
    let lines = input.lines().count();
    let line_len = input.lines().next().unwrap().len();

    let mut n = input.matches("XMAS").count() + input.matches("SAMX").count();

    // transpose grid and search
    let mut v = vec![String::new(); line_len];
    for line in input.lines() {
        for (j, char) in line.chars().enumerate() {
            v[j].push(char);
        }
    }
    n += v.iter().map(|l| l.matches("XMAS").count() + l.matches("SAMX").count()).sum::<usize>();

    // skew grid in both directions and search
    let mut v = vec![String::new(); line_len];
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            v[(j + i) % lines].push(char);
        }
    }
    // add a space where grid wraps around edge to avoid erroneous matches
    for (i, line) in v.iter_mut().enumerate() {
        line.insert(i + 1, ' ');
    }
    n += v.iter().map(|l| l.matches("XMAS").count() + l.matches("SAMX").count()).sum::<usize>();

    let mut v = vec![String::new(); line_len];
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            v[(j + i * (lines - 1)) % line_len].push(char);
        }
    }
    for (i, line) in v.iter_mut().enumerate() {
        line.insert(lines - i, ' ');
    }
    n += v.iter().map(|l| l.matches("XMAS").count() + l.matches("SAMX").count()).sum::<usize>();

    println!("{n}");
}

pub(crate) fn part2(input: String) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut n = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if x > 0 && y > 0 && y < grid.len() - 1 && x < grid[0].len() - 1
                && grid[y][x] == 'A'
                && ((grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S') || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M'))
                && ((grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S') || (grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M'))
            {
                n += 1;
            }
        }
    }
    println!("{n}");
}

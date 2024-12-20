use std::collections::{HashMap, VecDeque};

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Some((x, y));
                        '.'
                    },
                    'E' => '.',
                    c => c,
                })
                .collect()
        })
        .collect();
    (grid, start.unwrap())
}

fn map_path(track: &Vec<Vec<char>>, start: (usize, usize)) -> HashMap<(usize, usize), isize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((start, 0));
    visited.insert(start, 0);

    while let Some(((x, y), dist)) = queue.pop_front() {
        if x > 0 && track[y][x - 1] == '.' && !visited.contains_key(&(x - 1, y)) {
            visited.insert((x - 1, y), dist + 1);
            queue.push_back(((x - 1, y), dist + 1));
        }
        if x < track[0].len() - 1 && track[y][x + 1] == '.' && !visited.contains_key(&(x + 1, y)) {
            visited.insert((x + 1, y), dist + 1);
            queue.push_back(((x + 1, y), dist + 1));
        }
        if y > 0 && track[y - 1][x] == '.' && !visited.contains_key(&(x, y - 1)) {
            visited.insert((x, y - 1), dist + 1);
            queue.push_back(((x, y - 1), dist + 1));
        }
        if y < track.len() - 1 && track[y + 1][x] == '.' && !visited.contains_key(&(x, y + 1)) {
            visited.insert((x, y + 1), dist + 1);
            queue.push_back(((x, y + 1), dist + 1));
        }
    }

    visited

}

pub(crate) fn part1(input: String) {
    let (track, start) = parse(&input);
    let width = track[0].len();
    let height = track.len();
    let map = map_path(&track, start);

    let count = map
        .iter()
        .flat_map(|((x, y), dist)| {
            [(0, 2), (0, -2), (2, 0), (-2, 0)]
                .into_iter()
                .map(|(dx, dy)| {
                    let (nx, ny) = (*x as isize + dx, *y as isize + dy);
                    if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize
                        && map.get(&(nx as usize, ny as usize)).is_some_and(|&v| v - *dist - 2 >= 100)
                    {
                        1
                    } else {
                        0
                    }
                })
        })
        .sum::<usize>();

    println!("{count}");
}

pub(crate) fn part2(input: String) {
    let (track, start) = parse(&input);
    let width = track[0].len();
    let height = track.len();
    let map = map_path(&track, start);

    let coords: Vec<_> = map.keys().copied().collect();
    let mut count = 0;
    for i in 0..coords.len() {
        let pos1 @ (x1, y1) = coords[i];
        for j in i..coords.len() {
            let pos2 @ (x2, y2) = coords[j];
            let distance = x1.abs_diff(x2) + y1.abs_diff(y2);
            if distance <= 20 && map.get(&pos1).unwrap().abs_diff(*map.get(&pos2).unwrap()) - distance >= 100 {
                count += 1;
            }
        }
    }

    println!("{count}");
}

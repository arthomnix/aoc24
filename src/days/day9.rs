use std::collections::HashSet;

fn parse(input: String) -> Vec<(Option<i64>, i64)> {
    let mut id = 0i64;
    input.trim().chars().enumerate().map(|(i, c)| if i % 2 == 0 {
        let v = (Some(id), c as i64 - '0' as i64);
        id += 1;
        v
    } else {
        (None, c as i64 - '0' as i64)
    }).collect::<Vec<_>>()
}

pub(crate) fn part1(input: String) {
    let mut map = parse(input);

    let mut i = 0;
    let mut pos = 0;
    let mut checksum = 0;
    while i < map.len() {
        match map[i] {
            (Some(id), count) => for _ in 0..count {
                checksum += pos * id;
                pos += 1;
                map[i].1 -= 1;
            },
            (None, mut count) => while count > 0 {
                let last = map.iter_mut().rev().next();
                match last {
                    Some(&mut (None, _)) | Some(&mut (_, 0)) => {
                        map.pop();
                    },
                    Some((Some(id), c)) => {
                        *c -= 1;
                        checksum += pos * *id;
                        pos += 1;
                        count -= 1;
                    }
                    _ => break,
                }
            }
        }
        i += 1;
    }

    println!("{checksum}");
}

fn join_frees(map: Vec<(Option<i64>, i64)>, i: usize) -> (Vec<(Option<i64>, i64)>, usize) {
    let mut new = Vec::with_capacity(map.len());
    let mut n = 0;
    for (index, item) in map.into_iter().enumerate() {
        match (item, new.last_mut()) {
            ((None, 0), _) => if index <= i { n += 1 },
            ((None, count), Some((None, existing))) => {
                *existing += count;
                if index <= i { n += 1 };
            },
            _ => new.push(item),
        }
    }
    (new, n)
}

pub(crate) fn part2(input: String) {
    let mut map = parse(input);

    let mut seen = HashSet::new();
    let mut j = map.len() - 1;
    while j > 0 {
        match map[j] {
            (Some(id), count) => {
                if seen.insert(id) {
                    if let Some((index, empty)) = map.iter_mut().enumerate().filter(|(index, item)| item.0.is_none() && item.1 >= count && *index < j).next() {
                        empty.1 -= count;
                        let item = map[j];
                        map[j].0 = None;
                        map.insert(index, item);
                    } else {
                        j -= 1;
                    }
                } else {
                    j -= 1;
                }
            },
            (None, _) => j -= 1,
        }

        let n;
        (map, n) = join_frees(map, j);
        j -= n;
    }

    let mut pos = 0;
    let mut checksum = 0;
    for item in map {
        match item {
            (None, count) => for _ in 0..count {
                pos += 1;
            },
            (Some(id), count) => for _ in 0..count {
                checksum += id * pos;
                pos += 1;
            },
        }
    }

    println!("{checksum}");
}

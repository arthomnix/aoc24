use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn part1(input: String) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut split = line.split_whitespace();
        left.push(i32::from_str(split.next().unwrap()).unwrap());
        right.push(i32::from_str(split.next().unwrap()).unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    let mut total = 0;
    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }
    println!("{total}");
}

pub(crate) fn part2(input: String) {
    let mut left = vec![];
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        left.push(i32::from_str(split.next().unwrap()).unwrap());

        let right_num = i32::from_str(split.next().unwrap()).unwrap();
        if right.contains_key(&right_num) {
            *right.get_mut(&right_num).unwrap() += 1;
        } else {
            right.insert(right_num, 1);
        }
    }

    let mut total = 0;
    for num in left {
        total += num * *right.get(&num).unwrap_or(&0);
    }

    println!("{total}");
}

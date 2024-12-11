use std::collections::HashMap;

fn split_digits(num: i64) -> (i64, i64) {
    let digits = num.ilog10() + 1;
    let left = num / 10i64.pow(digits / 2);
    (left, num - left * 10i64.pow(digits / 2))
}

fn blink(stones: &mut HashMap<i64, i64>) {
    for (id, number) in stones.clone() {
        stones.entry(id).and_modify(|n| *n -= number);
        match id {
            0 => {
                stones.entry(1).and_modify(|n| *n += number).or_insert(number);
            },
            s if s.ilog10() % 2 == 1 => {
                let (l, r) = split_digits(s);
                stones.entry(l).and_modify(|n| *n += number).or_insert(number);
                stones.entry(r).and_modify(|n| *n += number).or_insert(number);
            },
            s => {
                stones.entry(s * 2024).and_modify(|n| *n += number).or_insert(number);
            },
        }
    }
}

fn day(input: String, part2: bool) {
    let mut stones = input.trim().split_whitespace().map(|n| (n.parse().unwrap(), 1)).collect::<HashMap<i64, i64>>();
    for i in 0..if part2 { 75 } else { 25 } {
        blink(&mut stones);
    }
    println!("{}", stones.values().sum::<i64>());
}

pub(crate) fn part1(input: String) {
    day(input, false);
}

pub(crate) fn part2(input: String) {
    day(input, true);
}

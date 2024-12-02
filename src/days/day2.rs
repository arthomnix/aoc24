#[inline(always)]
pub(crate) fn day(input: String, part2: bool) {
    let count = input
        .lines()
        .map(|line| {
            let split: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            // dirty, but still very fast on input
            let mut split_v = vec![split.clone()];
            if part2 {
                for i in 0..split.len() {
                    let mut v = split.clone();
                    v.remove(i);
                    split_v.push(v);
                }
            }

            if split_v.iter().any(|split| {
                let diffs: Vec<i32> = split.windows(2)
                    .map(|w| w[1] - w[0])
                    .collect();

                let is_safe_increasing = diffs.iter().all(|&x| x >= 1 && x <= 3);
                let is_safe_decreasing = diffs.iter().all(|&x| x <= -1 && x >= -3);
                is_safe_increasing || is_safe_decreasing
            }) { 1 } else { 0 }

        }).sum::<i32>();
    println!("{count}");
}

pub(crate) fn part1(input: String) {
    day(input, false);
}

pub(crate) fn part2(input: String) {
    day(input, true);
}

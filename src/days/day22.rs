use std::collections::HashMap;

fn next(mut secret: i64) -> i64 {
    secret = ((secret << 6 ) ^ secret) & 0xffffff;
    secret = ((secret >> 5 ) ^ secret) & 0xffffff;
    secret = ((secret << 11) ^ secret) & 0xffffff;
    secret
}

pub(crate) fn part1(input: String) {
    let res = input
        .lines()
        .map(|l| {
            let mut secret = l.parse::<i64>().unwrap();
            for _ in 0..2000 {
                secret = next(secret);
            }
            secret
        })
        .sum::<i64>();
    println!("{res}");
}

pub(crate) fn part2(input: String) {
    let prices = input
        .lines()
        .map(|l| {
            let mut secret = l.parse::<i64>().unwrap();
            (0..=2000).map(|_| {
                let res = secret % 10;
                secret = next(secret);
                res
            }).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let changes = prices
        .iter()
        .map(|p| {
            p
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let bananas = prices
        .iter()
        .zip(changes.iter())
        .map(|(p, c)| {
            c
                .windows(4)
                .enumerate()
                .rev()
                .map(|(i, w)| {
                    ([w[0], w[1], w[2], w[3]], p[i + 4])
                })
                .collect::<HashMap<_, _>>()
        })
        .reduce(|mut a, b| {
            for (k, v) in b {
                a.entry(k).and_modify(|n| *n += v).or_insert(v);
            }
            a
        })
        .unwrap()
        .into_values()
        .max()
        .unwrap();

    println!("{bananas}");
}

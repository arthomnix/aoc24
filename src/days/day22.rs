use rayon::prelude::*;

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

    let bananas = (-9..=9).flat_map(|a| {
        let prices = &prices;
        let changes = &changes;
        dbg!(a);
        (-9..=9).flat_map(move |b| {
            dbg!(b);
            (-9..=9).flat_map(move |c| {
                (-9..=9).map(move |d| {
                    changes
                        .par_iter()
                        .enumerate()
                        .filter_map(|(i, ch)| {
                            ch
                                .windows(4)
                                .enumerate()
                                .find(|&(_, w)| w == &[a, b, c, d])
                                .map(|(j, _)| prices[i][j + 4])
                        })
                        .sum::<i64>()
                })
            })
        })
    })
        .max()
        .unwrap();

    println!("{bananas}");
}

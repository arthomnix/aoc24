use std::arch::x86_64::*;
use std::collections::HashMap;

fn next(mut secret: i64) -> i64 {
    secret = ((secret << 6 ) ^ secret) & 0xffffff;
    secret = ((secret >> 5 ) ^ secret) & 0xffffff;
    secret = ((secret << 11) ^ secret) & 0xffffff;
    secret
}

#[repr(align(64))]
struct Aligned([i64; 8]);

pub(crate) fn part1(input: String) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut a = Aligned([0; 8]);
    let mask = Aligned([0xffffff; 8]);
    let mask = unsafe { _mm512_load_epi64(&mask.0 as *const i64) };
    let res = lines
        .chunks(8)
        .map(|chunks| {
            if chunks.len() == 8 {
                for i in 0..8 {
                    a.0[i] = chunks[i].parse().unwrap();
                }
                unsafe {
                    let mut secrets = _mm512_load_epi64(&a.0 as *const i64);
                    for _ in 0..2000 {
                        let secrets2 = secrets.clone();
                        secrets = _mm512_slli_epi64::<6>(secrets);
                        secrets = _mm512_xor_epi64(secrets, secrets2);
                        secrets = _mm512_and_epi64(secrets, mask);
                        let secrets2 = secrets.clone();
                        secrets = _mm512_srli_epi64::<5>(secrets);
                        secrets = _mm512_xor_epi64(secrets, secrets2);
                        let secrets2 = secrets.clone();
                        secrets = _mm512_slli_epi64::<11>(secrets);
                        secrets = _mm512_xor_epi64(secrets, secrets2);
                        secrets = _mm512_and_epi64(secrets, mask);
                    }
                    _mm512_reduce_add_epi64(secrets)
                }
            } else {
                chunks
                    .iter()
                    .map(|&l| {
                        let mut secret = l.parse::<i64>().unwrap();
                        for _ in 0..2000 {
                            secret = next(secret);
                        }
                        secret
                    })
                    .sum::<i64>()
            }
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

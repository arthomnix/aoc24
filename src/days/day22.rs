use std::arch::x86_64::*;
use std::collections::HashMap;

#[repr(align(64))]
struct Aligned([i32; 16]);

pub(crate) fn part1(input: String) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut a = Aligned([0; 16]);
    let mask = Aligned([0xffffff; 16]);
    let mask = unsafe { _mm512_load_epi32(&mask.0 as *const i32) };
    let res = lines
        .chunks(16)
        .map(|chunks| {
            for i in 0..16 {
                a.0[i] = chunks.get(i).map(|&s| s.parse().unwrap()).unwrap_or(0);
            }
            let k = ((1 << chunks.len()) - 1) as __mmask16;
            unsafe {
                let mut secrets = _mm512_load_epi32(&a.0 as *const i32);
                for _ in 0..2000 {
                    let secrets2 = secrets.clone();
                    secrets = _mm512_slli_epi32::<6>(secrets);
                    secrets = _mm512_xor_epi32(secrets, secrets2);
                    secrets = _mm512_and_epi32(secrets, mask);
                    let secrets2 = secrets.clone();
                    secrets = _mm512_srli_epi32::<5>(secrets);
                    secrets = _mm512_xor_epi32(secrets, secrets2);
                    let secrets2 = secrets.clone();
                    secrets = _mm512_slli_epi32::<11>(secrets);
                    secrets = _mm512_xor_epi32(secrets, secrets2);
                    secrets = _mm512_and_epi32(secrets, mask);
                }
                _mm512_mask_reduce_add_epi32(k, secrets) as i64
            }
        })
        .sum::<i64>();
    println!("{res}");
}

fn next(mut secret: i64) -> i64 {
    secret = ((secret << 6 ) ^ secret) & 0xffffff;
    secret = ((secret >> 5 ) ^ secret) & 0xffffff;
    secret = ((secret << 11) ^ secret) & 0xffffff;
    secret
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

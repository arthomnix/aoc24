use std::arch::x86_64::*;
use std::collections::HashMap;

#[repr(align(64))]
#[derive(Copy, Clone, Debug)]
struct Aligned([i32; 16]);

pub(crate) fn part1(input: String) {
    let lines = input.lines().collect::<Vec<_>>();

    // AVX-512 GFNI solution, based on work by @voltara88 on the AoC Discord
    // https://voltara.org/advent/2024/Advent%20of%20Code%202024%20Day%2022%20Part%201.pdf

    let matrices_0 = unsafe {
        _mm512_set_epi64(
            0xaf888fd0c9634130_u64 as i64, 0xaf888fd0c9634130_u64 as i64,
            0x1917419c62e210ad_u64 as i64, 0x1917419c62e210ad_u64 as i64,
            0x16463dbaced076c1_u64 as i64, 0x16463dbaced076c1_u64 as i64,
            0x0000000000000000_u64 as i64, 0x0000000000000000_u64 as i64,
        )
    };

    let matrices_1 = unsafe {
        _mm512_set_epi64(
            0xe7c34220149b7728_u64 as i64, 0xe7c34220149b7728_u64 as i64,
            0x553337b6adec81cf_u64 as i64, 0x553337b6adec81cf_u64 as i64,
            0x46714f6cbc284527_u64 as i64, 0x46714f6cbc284527_u64 as i64,
            0x0000000000000000_u64 as i64, 0x0000000000000000_u64 as i64,
         )
    };
    let matrices_2 = unsafe {
        _mm512_set_epi64(
            0xb00193b6071c8868_u64 as i64, 0xb00193b6071c8868_u64 as i64,
            0x545796a28c730795_u64 as i64, 0x545796a28c730795_u64 as i64,
            0x718462a50da6d92d_u64 as i64, 0x718462a50da6d92d_u64 as i64,
            0x0000000000000000_u64 as i64, 0x0000000000000000_u64 as i64,
        )
    };

    let p_low = unsafe { _mm512_set4_epi32(0x0004080c, 0x1014181c, 0x2024282c, 0x3034383c) };
    let p_mid = unsafe { _mm512_set4_epi32(0x0105090d, 0x1115191d, 0x2125292d, 0x3135393d) };
    let p_hi  = unsafe { _mm512_set4_epi32(0x02060a0e, 0x12161a1e, 0x22262a2e, 0x32363a3e) };

    let p_rev = unsafe {
        _mm512_set_epi64(
            0x0010203000112131, 0x0012223200132333,
            0x0014243400152535, 0x0016263600172737,
            0x0018283800192939, 0x001a2a3a001b2b3b,
            0x001c2c3c001d2d3d, 0x001e2e3e001f2f3f,
        )
    };

    let mut a = Aligned([0; 16]);
    let mut result = unsafe { _mm512_setzero_epi32() };

    for chunks in lines.chunks(16) {
        for i in 0..16 {
            a.0[i] = chunks.get(i).map(|&s| s.parse().unwrap()).unwrap_or(0);
        }

        unsafe {
            let secrets = _mm512_load_si512(&a.0 as *const i32);
            let secrets_low = _mm512_permutexvar_epi8(p_low, secrets);
            let secrets_mid = _mm512_permutexvar_epi8(p_mid, secrets);
            let secrets_hi  = _mm512_permutexvar_epi8(p_hi,  secrets);
            let a = _mm512_gf2p8affine_epi64_epi8::<0>(secrets_low, matrices_0);
            let b = _mm512_gf2p8affine_epi64_epi8::<0>(secrets_mid, matrices_1);
            let c = _mm512_gf2p8affine_epi64_epi8::<0>(secrets_hi,  matrices_2);
            let d = _mm512_ternarylogic_epi64::<0x96>(a, b, c);
            let r = _mm512_permutexvar_epi8(p_rev, d);
            result = _mm512_add_epi32(result, r);
        }
    }

    let res = unsafe {
        let lo = _mm512_unpacklo_epi32(result, _mm512_setzero_epi32());
        let hi = _mm512_unpackhi_epi32(result, _mm512_setzero_epi32());
        _mm512_reduce_add_epi64(_mm512_add_epi64(lo, hi))
    };

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

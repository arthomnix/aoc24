use std::arch::x86_64::*;
use std::collections::HashMap;
use std::str::FromStr;

#[repr(align(64))]
struct Aligned([i32; 1008]);

pub(crate) fn part1(input: String) {
    let mut left = Aligned([0; 1008]);
    let mut right = Aligned([0; 1008]);

    for (i, line) in input.lines().enumerate() {
        left.0[i] = line[..5].parse().unwrap();
        right.0[i] = line[8..].parse().unwrap();
    }

    left.0.sort_unstable();
    right.0.sort_unstable();

    let mut total = 0;
    for i in 0..63 {
        let j = i << 4;
        unsafe {
            let mut ll = _mm512_load_epi32(&left.0[j] as *const i32);
            let rl = _mm512_load_epi32(&right.0[j] as *const i32);
            ll = _mm512_sub_epi32(ll, rl);
            ll = _mm512_abs_epi32(ll);
            total += _mm512_reduce_add_epi32(ll);
        }
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

use std::cmp::Ordering;
use std::collections::HashSet;

fn parse(input: String) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let (r, u) = input.split_once("\n\n").unwrap();
    let rules = r
        .lines()
        .map(|l| {
            let (left, right) = l.split_once('|').unwrap();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .collect::<HashSet<_>>();
    let updates = u
        .lines()
        .map(|l| l
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (rules, updates)
}

pub(crate) fn part1(input: String) {
    let (rules, updates) = parse(input);

    let mut total = 0;
    'updates: for update in updates {
        for &(l, r) in &rules {
            let l_pos = update.iter().position(|&x| x == l);
            let r_pos = update.iter().position(|&x| x == r);
            if let (Some(lp), Some(rp)) = (l_pos, r_pos) {
                if lp > rp {
                    continue 'updates;
                }
            }
        }
        total += update[(update.len() + 1) / 2 - 1];
    }
    println!("{total}");
}

pub(crate) fn part2(input: String) {
    let (rules, updates) = parse(input);
    let mut sorted_updates = updates.clone();
    for update in sorted_updates.iter_mut() {
        update.sort_unstable_by(|&l, &r| match (l, r) {
            _ if rules.contains(&(l, r)) => Ordering::Less,
            _ if rules.contains(&(r, l)) => Ordering::Greater,
            _ => Ordering::Equal,
        })
    };
    println!(
        "{}",
        updates.into_iter().zip(sorted_updates.into_iter())
            .filter_map(|(l, r)| if l != r {
                Some(r[(r.len() + 1) / 2 - 1])
            } else {
                None
            })
            .sum::<i32>()
    );
}
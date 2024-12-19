use std::collections::HashMap;
use regex::Regex;

fn parse(input: &str) -> (Vec<&str>, impl Iterator<Item = &str>) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let patterns: Vec<_> = a.split(", ").collect();
    let designs = b.lines();
    (patterns, designs)
}

pub(crate) fn part1(input: String) {
    let (patterns, designs) = parse(&input);
    let regex = format!("^({})+$", patterns.join("|"));
    let re = Regex::new(&regex).unwrap();
    println!("{}", designs.filter(|&s| re.is_match(&s)).count());
}

fn search(pattern: &str, patterns: &[&str], memo: &mut HashMap<String, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(res) = memo.get(pattern) {
        return *res;
    }

    let mut count = 0;

    for &p in patterns {
        if pattern.starts_with(p) {
            count += search(&pattern[p.len()..], patterns, memo);
        }
    }

    memo.insert(pattern.to_string(), count);
    count
}

pub(crate) fn part2(input: String) {
    let (patterns, designs) = parse(&input);
    let mut memo = HashMap::new();
    println!("{}", designs.map(|d| search(d, &patterns, &mut memo)).sum::<usize>());
}

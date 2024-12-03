use regex::Regex;

pub(crate) fn part1(input: String) {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    println!(
        "{}",
        regex
            .captures_iter(&input)
            .map(|cap| {
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap() * cap.get(2).unwrap().as_str().parse::<i32>().unwrap()
            })
            .sum::<i32>()
    );
}

pub(crate) fn part2(input: String) {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut doing = true;
    println!(
        "{}",
        regex
            .captures_iter(&input)
            .map(|cap| match cap.get(0).unwrap().as_str() {
                "do()" => { doing = true; 0 },
                "don't()" => { doing = false; 0 },
                _ => if doing { cap.get(1).unwrap().as_str().parse::<i32>().unwrap() * cap.get(2).unwrap().as_str().parse::<i32>().unwrap() } else { 0 },
            })
            .sum::<i32>()
    );
}

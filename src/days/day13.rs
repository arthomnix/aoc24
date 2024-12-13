struct Machine {
    xa: i64,
    ya: i64,
    xb: i64,
    yb: i64,
    x: i64,
    y: i64,
}

impl Machine {
    fn solve(&self) -> Option<(i64, i64)> {
        let an = self.yb * self.x - self.xb * self.y;
        let ad = self.xa * self.yb - self.xb * self.ya;
        let bn = self.ya * self.x - self.xa * self.y;
        let bd = self.ya * self.xb - self.xa * self.yb;
        if an % ad == 0 && bn % bd == 0 {
            Some((an / ad, bn / bd))
        } else {
            None
        }
    }
}

fn parse<'a>(input: &'a str, part2: bool) -> impl Iterator<Item = Machine> + use<'a> {
    input.split("\n\n")
        .map(move |block| {
            let mut lines = block.lines();
            let mut a = lines.next().unwrap().split(&['+', ',']).filter_map(|s| s.parse::<i64>().ok());
            let mut b = lines.next().unwrap().split(&['+', ',']).filter_map(|s| s.parse::<i64>().ok());
            let mut p = lines.next().unwrap().split(&['=', ',']).filter_map(|s| s.parse::<i64>().ok());
            let offset = if part2 { 10000000000000 } else { 0 };
            Machine {
                xa: a.next().unwrap(),
                ya: a.next().unwrap(),
                xb: b.next().unwrap(),
                yb: b.next().unwrap(),
                x: p.next().unwrap() + offset,
                y: p.next().unwrap() + offset,
            }
        })
}

fn day(input: String, part2: bool) {
    println!(
        "{}",
        parse(&input, part2).map(|machine| {
            let (a, b) = machine.solve().unwrap_or((0, 0));
            if part2 || (a <= 100 && b <= 100) {
                a * 3 + b
            } else {
                0
            }
        }).sum::<i64>()
    );
}

pub(crate) fn part1(input: String) {
    day(input, false);
}

pub(crate) fn part2(input: String) {
    day(input, true);
}

#[derive(Clone, Debug)]
struct Computer {
    memory: Vec<u8>,
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
    output_buffer: Vec<u8>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let (regs, program) = input.split_once("\n\n").unwrap();
        let mut rl = regs.lines();
        let a = rl.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let b = rl.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let c = rl.next().unwrap().split(" ").last().unwrap().parse().unwrap();

        let (_, mem) = program.trim().split_once(" ").unwrap();
        let memory = mem.split(",").map(|s| s.parse().unwrap()).collect();

        Self {
            memory,
            ip: 0,
            a,
            b,
            c,
            output_buffer: vec![],
        }
    }

    fn reset(&mut self) {
        self.output_buffer.clear();
        self.ip = 0;
    }

    fn combo(&self, n: u8) -> u64 {
        match n {
            0..=3 => n as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo operand"),
        }
    }

    fn tick(&mut self) -> bool {
        if self.ip >= self.memory.len() {
            return false;
        }

        let opcode = self.memory[self.ip];
        let operand = self.memory[self.ip + 1];
        let mut should_increment = true;

        match opcode {
            // adv
            0 => self.a /= 1 << self.combo(operand),
            // bxl
            1 => self.b ^= operand as u64,
            // bst
            2 => self.b = self.combo(operand) & 0b111,
            // jnz
            3 => if self.a != 0 {
                self.ip = operand as usize;
                should_increment = false;
            },
            // bxc
            4 => self.b ^= self.c,
            // out
            5 => self.output_buffer.push((self.combo(operand) & 0b111) as u8),
            // bdv
            6 => self.b = self.a / (1 << self.combo(operand)),
            // cdv
            7 => self.c = self.a / (1 << self.combo(operand)),
            _ => panic!("illegal instruction"),
        }

        if should_increment {
            self.ip += 2;
        }

        true
    }

    fn run(&mut self) {
        while self.tick() {}
    }
}

pub(crate) fn part1(input: String) {
    let mut computer = Computer::parse(&input);
    computer.run();
    println!("{}", computer.output_buffer.into_iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
}

fn search(computer: &mut Computer, a: u64, i: usize) -> Option<u64> {
    if i == computer.memory.len() {
        return Some(a);
    }

    computer.reset();

    let target_digit = *computer.memory.iter().nth_back(i).unwrap();
    for x in 0..8 {
        computer.a = (a << 3) + x;
        computer.run();
        if computer.output_buffer[0] == target_digit {
            if let Some(n) = search(computer, (a << 3) + x, i + 1) {
                return Some(n);
            }
        }
        computer.reset();
    }

    None
}

pub(crate) fn part2(input: String) {
    let mut computer = Computer::parse(&input);

    println!("{}", search(&mut computer, 0, 0).unwrap());
}

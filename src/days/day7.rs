use std::ops::{BitAndAssign, BitXorAssign};
use Operator::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl BitXorAssign for Operator {
    fn bitxor_assign(&mut self, rhs: Self) {
        match (*self == Mul) ^ (rhs == Mul) {
            true => *self = Mul,
            false => *self = Add,
        }
    }
}

impl BitAndAssign for Operator {
    fn bitand_assign(&mut self, rhs: Self) {
        match (*self == Mul) && (rhs == Mul) {
            true => *self = Mul,
            false => *self = Add,
        }
    }
}

impl Operator {
    fn add_ternary(&self, rhs: &Self) -> (Self, Self) {
        match (self, rhs) {
            (Add, Add) => (Add, Add),
            (Add, Mul) => (Add, Mul),
            (Add, Concat) => (Add, Concat),
            (Mul, Add) => (Add, Mul),
            (Mul, Mul) => (Add, Concat),
            (Mul, Concat) => (Mul, Add),
            (Concat, Add) => (Add, Concat),
            (Concat, Mul) => (Mul, Add),
            (Concat, Concat) => (Mul, Mul),
        }
    }
}

#[derive(Clone, Debug)]
struct Ops(Vec<Operator>);

impl Ops {
    fn next(&mut self) -> bool {
        let mut carry = Mul;
        for op in self.0.iter_mut() {
            let original_op = *op;
            *op ^= carry;
            carry &= original_op;
        }
        carry == Add
    }

    fn next_concat(&mut self) -> bool {
        let mut carry = Mul;
        for op in self.0.iter_mut() {
            (carry, *op) = op.add_ternary(&carry);
        }
        carry == Add
    }
}

#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn parse(input: &str) -> Self {
        let (result, numbers) = input.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let numbers = numbers.split_whitespace().map(|n| n.parse().unwrap()).collect();
        Self {
            result,
            numbers,
        }
    }

    fn are_ops_valid(&self, ops: &Ops) -> bool {
        let mut iter = self.numbers.iter();
        let first = *iter.next().unwrap();
        iter.enumerate().fold(first, |a, (i, &n)| match ops.0[i] {
            Add => a + n,
            Mul => a * n,
            Concat => Self::concat_digits(a, n)
        }) == self.result
    }

    fn concat_digits(a: i64, b: i64) -> i64 {
        let mut count = 0;
        let mut tmp = b;
        while tmp != 0 {
            count += 1;
            tmp /= 10;
        }
        a * 10i64.pow(count) + b
    }

    fn is_valid(&self, concat: bool) -> bool {
        let mut ops = Ops(vec![Add; self.numbers.len() - 1]);
        loop {
            if self.are_ops_valid(&ops) {
                return true;
            }

            if (concat && !ops.next_concat()) || (!concat && !ops.next()) {
                return false;
            }
        }
    }
}

pub(crate) fn part1(input: String) {
    println!("{}",
            input.lines().filter_map(|l| {
                let equation = Equation::parse(l);
                if equation.is_valid(false) { Some(equation.result) } else { None }
            }).sum::<i64>()
    );
}

pub(crate) fn part2(input: String) {
    println!("{}",
             input.lines().filter_map(|l| {
                 let equation = Equation::parse(l);
                 if equation.is_valid(true) { Some(equation.result) } else { None }
             }).sum::<i64>()
    );
}

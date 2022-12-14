use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#,
    )
}

enum Var {
    Old,
    Lit(u64),
}

impl Var {
    fn from(val: &str) -> Self {
        match val {
            "old" => Self::Old,
            n => Self::Lit(n.parse().unwrap()),
        }
    }
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn from(val: &str) -> Self {
        match val {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: (Var, Op, Var),
    test_divisible_by: u64,
    on_true: usize,
    on_false: usize,
    inspected_items: u64,
}

impl Monkey {
    fn from(lines: &[String]) -> Self {
        let (_, starting_items) = lines[1].split_once(": ").unwrap();
        let items = starting_items
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let (_, operation) = lines[2].split_once(" = ").unwrap();
        let parts: Vec<_> = operation.split(' ').collect();
        let operation = (Var::from(parts[0]), Op::from(parts[1]), Var::from(parts[2]));

        let test_divisible_by = lines[3].split(' ').last().unwrap().parse().unwrap();

        let on_true = lines[4].split(' ').last().unwrap().parse().unwrap();
        let on_false = lines[5].split(' ').last().unwrap().parse().unwrap();

        Monkey {
            items,
            operation,
            test_divisible_by,
            on_true,
            on_false,
            inspected_items: 0,
        }
    }

    fn apply_operation(&mut self, divide: bool) -> u64 {
        self.inspected_items += 1;
        let mut old = self.items.remove(0);
        let left = match &self.operation.0 {
            Var::Old => &old,
            Var::Lit(n) => n,
        };
        let right = match &self.operation.2 {
            Var::Old => &old,
            Var::Lit(n) => n,
        };
        old = match self.operation.1 {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        };
        if divide {
            old /= 3;
        }
        old
    }
}

pub fn solve1() -> u64 {
    let mut monkeys: Vec<_> = read_lines("/home/elliotbobrow/Downloads/day11.txt")
        .split(|line| line.is_empty())
        .map(Monkey::from)
        .collect();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let new = monkeys[i].apply_operation(true);
                if new % monkeys[i].test_divisible_by == 0 {
                    let on_true = monkeys[i].on_true;
                    monkeys[on_true].items.push(new);
                } else {
                    let on_false = monkeys[i].on_false;
                    monkeys[on_false].items.push(new);
                }
            }
        }
    }

    let mut inspected: Vec<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspected_items)
        .collect();
    inspected.sort_by(|a, b| b.cmp(a));
    inspected[0] * inspected[1]
}

pub fn solve2() -> u64 {
    let mut monkeys: Vec<_> = read_lines("/home/elliotbobrow/Downloads/day11.txt")
        .split(|line| line.is_empty())
        .map(Monkey::from)
        .collect();
    // :| thanks https://www.reddit.com/r/adventofcode/comments/zizi43/comment/iztt8mx/?utm_source=share&utm_medium=web2x&context=3
    let cycle_length: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let mut new = monkeys[i].apply_operation(false);
                new %= cycle_length;
                if new % monkeys[i].test_divisible_by == 0 {
                    let on_true = monkeys[i].on_true;
                    monkeys[on_true].items.push(new);
                } else {
                    let on_false = monkeys[i].on_false;
                    monkeys[on_false].items.push(new);
                }
            }
        }
    }

    let mut inspected: Vec<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspected_items)
        .collect();
    inspected.sort_by(|a, b| b.cmp(a));
    inspected[0] * inspected[1]
}

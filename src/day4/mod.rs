use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#,
    )
}

fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
    (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)
}

fn between(n: u32, range: (u32, u32)) -> bool {
    n >= range.0 && n <= range.1
}

pub fn solve1() -> usize {
    // sample()
    read_lines("/home/elliotbobrow/Downloads/day4.txt")
        .iter()
        .filter(|line| {
            let range = |elf: &str| -> (u32, u32) {
                let (a, b) = elf.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            };
            let (a, b) = line.split_once(',').unwrap();
            contains(range(a), range(b))
        })
        .count()
}

pub fn solve2() -> usize {
    // sample()
    read_lines("/home/elliotbobrow/Downloads/day4.txt")
        .iter()
        .filter(|line| {
            let range = |elf: &str| -> (u32, u32) {
                let (a, b) = elf.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            };
            let (a, b) = line.split_once(',').unwrap();
            let (a, b) = (range(a), range(b));
            between(a.0, b) || between(a.1, b) || contains(a, b)
        })
        .count()
}

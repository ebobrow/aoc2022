use std::collections::HashSet;

use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#,
    )
}

pub fn solve1() -> u32 {
    read_lines("/home/elliotbobrow/Downloads/day3.txt")
        .iter()
        .map(|line| {
            let (fst, snd) = line.split_at(line.len() / 2);
            if let Some(&c) = HashSet::<char>::from_iter(fst.chars())
                .intersection(&HashSet::from_iter(snd.chars()))
                .next()
            {
                let x: u32 = c.into();
                if x > 96 {
                    return x - 96;
                } else {
                    return x - 38;
                }
            }
            unreachable!()
        })
        .sum()
}

pub fn solve2() -> u32 {
    read_lines("/home/elliotbobrow/Downloads/day3.txt")
        .chunks(3)
        .map(|group| {
            if let Some(c) = group
                .iter()
                .map(|x| HashSet::from_iter(x.chars()))
                .reduce(|acc, x| acc.intersection(&x).cloned().collect::<HashSet<_>>())
                .unwrap()
                .into_iter()
                .next()
            {
                let x: u32 = c.into();
                if x > 96 {
                    return x - 96;
                } else {
                    return x - 38;
                }
            }
            unreachable!()
        })
        .sum()
}

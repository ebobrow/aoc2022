use crate::util::read_lines;

pub fn solve1() -> u32 {
    let lines = read_lines("/home/elliotbobrow/Downloads/day1.txt");
    lines
        .split(String::is_empty)
        .map(|group| {
            group
                .iter()
                .map(|x| str::parse::<u32>(x).unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn solve2() -> u32 {
    let lines = read_lines("/home/elliotbobrow/Downloads/day1.txt");
    let mut totals: Vec<u32> = lines
        .split(String::is_empty)
        .map(|group| group.iter().map(|x| str::parse::<u32>(x).unwrap()).sum())
        .collect();
    totals.sort();
    totals.reverse();
    totals[..3].iter().sum()
}

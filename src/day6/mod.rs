use std::{collections::HashSet, fs};

pub fn solve(n: usize) -> usize {
    // let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let data = fs::read_to_string("/home/elliotbobrow/Downloads/day6.txt").unwrap();
    for i in n..data.len() {
        if HashSet::<char>::from_iter(&mut data[i - n..i].chars()).len() == n {
            return i;
        }
    }
    unreachable!()
}

pub fn solve1() -> usize {
    solve(4)
}

pub fn solve2() -> usize {
    solve(14)
}

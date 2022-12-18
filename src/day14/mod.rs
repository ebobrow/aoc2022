use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#,
    )
}

fn add_coord(coords: &mut Vec<(u32, u32)>, x: u32, y: u32) {
    if !coords.contains(&(x, y)) {
        coords.push((x, y));
    }
}

pub fn solve1() -> u32 {
    let mut coords = Vec::new();
    // for line in sample() {
    for line in read_lines("/home/elliotbobrow/Downloads/day14.txt") {
        let _ = line
            .split(" -> ")
            .map(|coord| {
                let coords: Vec<_> = coord
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();
                (coords[0], coords[1])
            })
            .reduce(|acc, coord| {
                if acc.0 == coord.0 {
                    let start = std::cmp::min(acc.1, coord.1);
                    let end = std::cmp::max(acc.1, coord.1);
                    for y in start..=end {
                        add_coord(&mut coords, acc.0, y);
                    }
                    coord
                } else {
                    let start = std::cmp::min(acc.0, coord.0);
                    let end = std::cmp::max(acc.0, coord.0);
                    for x in start..=end {
                        add_coord(&mut coords, x, acc.1);
                    }
                    coord
                }
            });
    }
    let lowest = coords.iter().map(|(_, y)| y).max().unwrap().clone();
    let mut grains = 0;
    loop {
        grains += 1;
        let mut cur_pos = (500, 0);
        loop {
            if cur_pos.1 > lowest {
                return grains - 1;
            }
            if !coords.contains(&(cur_pos.0, cur_pos.1 + 1)) {
                cur_pos.1 += 1;
            } else if !coords.contains(&(cur_pos.0 - 1, cur_pos.1 + 1)) {
                cur_pos = (cur_pos.0 - 1, cur_pos.1 + 1);
            } else if !coords.contains(&(cur_pos.0 + 1, cur_pos.1 + 1)) {
                cur_pos = (cur_pos.0 + 1, cur_pos.1 + 1);
            } else {
                coords.push(cur_pos);
                break;
            }
        }
    }
}

pub fn solve2() -> u32 {
    let mut coords = Vec::new();
    // for line in sample() {
    for line in read_lines("/home/elliotbobrow/Downloads/day14.txt") {
        let _ = line
            .split(" -> ")
            .map(|coord| {
                let coords: Vec<_> = coord
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();
                (coords[0], coords[1])
            })
            .reduce(|acc, coord| {
                if acc.0 == coord.0 {
                    let start = std::cmp::min(acc.1, coord.1);
                    let end = std::cmp::max(acc.1, coord.1);
                    for y in start..=end {
                        add_coord(&mut coords, acc.0, y);
                    }
                    coord
                } else {
                    let start = std::cmp::min(acc.0, coord.0);
                    let end = std::cmp::max(acc.0, coord.0);
                    for x in start..=end {
                        add_coord(&mut coords, x, acc.1);
                    }
                    coord
                }
            });
    }
    let lowest = coords.iter().map(|(_, y)| y).max().unwrap().clone();
    let floor = lowest + 2;
    let mut grains = 0;
    loop {
        grains += 1;
        let mut cur_pos = (500, 0);
        loop {
            if cur_pos.1 + 1 == floor {
                coords.push(cur_pos);
                break;
            }
            if !coords.contains(&(cur_pos.0, cur_pos.1 + 1)) {
                cur_pos.1 += 1;
            } else if !coords.contains(&(cur_pos.0 - 1, cur_pos.1 + 1)) {
                cur_pos = (cur_pos.0 - 1, cur_pos.1 + 1);
            } else if !coords.contains(&(cur_pos.0 + 1, cur_pos.1 + 1)) {
                cur_pos = (cur_pos.0 + 1, cur_pos.1 + 1);
            } else {
                coords.push(cur_pos);
                break;
            }
        }
        if cur_pos == (500, 0) {
            return grains;
        }
    }
}

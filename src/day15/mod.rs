use std::collections::HashSet;

use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#,
    )
}

fn manhattan_dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn from(line: impl ToString) -> Self {
        let line = &line.to_string()[12..];
        let (pos, beacon) = line.split_once(": closest beacon is at x=").unwrap();
        let get_coords = |s: &str| -> (i64, i64) {
            let (x, y) = s.split_once(", y=").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        };
        Self {
            pos: get_coords(pos),
            beacon: get_coords(beacon),
        }
    }
}

pub fn solve1() -> i64 {
    // let data = sample();
    let data = read_lines("/home/elliotbobrow/Downloads/day15.txt");
    // let row_num = 10;
    let row_num = 2000000;
    let sensors: Vec<_> = data.iter().map(Sensor::from).collect();
    let beacons: Vec<_> = sensors
        .iter()
        .filter(|sensor| sensor.beacon.1 == row_num)
        .map(|sensor| sensor.beacon.0)
        .collect();
    let mut ranges = Vec::new();
    for sensor in sensors {
        let d = manhattan_dist(sensor.pos, sensor.beacon);
        let end = d - (sensor.pos.1 - row_num).abs();
        if end >= 0 {
            ranges.push((sensor.pos.0 - end, sensor.pos.0 + end));
        }
    }
    ranges.sort_by_key(|&(s, _)| s);
    let mut whatever = vec![ranges[0]];
    for next in &ranges[1..] {
        let cur = whatever.remove(0);
        if cur.1 < next.0 {
            whatever.push(cur);
            whatever.push(*next);
        } else if next.1 >= cur.1 {
            whatever.push((cur.0, next.1));
        } else {
            whatever.push(cur);
        }
    }
    let exclude = beacons
        .iter()
        .filter(|x| {
            !whatever
                .iter()
                .any(|range| x >= &&range.0 && x <= &&range.1)
        })
        .count();
    whatever.iter().map(|(a, b)| b - a).sum::<i64>() - exclude as i64
}

pub fn solve2() -> i64 {
    // let data = sample();
    let data = read_lines("/home/elliotbobrow/Downloads/day15.txt");
    // let max = 20;
    let max = 4000000;
    let sensors: Vec<_> = data.iter().map(Sensor::from).collect();
    for row_num in 0..=max {
        let mut ranges = Vec::new();
        for sensor in &sensors {
            let d = manhattan_dist(sensor.pos, sensor.beacon);
            let end = d - (sensor.pos.1 - row_num).abs();
            if end >= 0 {
                let min_end = 0.max(sensor.pos.0 - end);
                let max_end = max.min(sensor.pos.0 + end);
                ranges.push((min_end, max_end));
            }
        }
        ranges.sort_by_key(|&(s, _)| s);
        let mut whatever = vec![ranges[0]];
        for next in &ranges[1..] {
            // let cur = whatever.remove(0);
            let cur = whatever.pop().unwrap();
            if cur.1 + 1 < next.0 {
                whatever.push(cur);
                whatever.push(*next);
            } else if next.1 > cur.1 {
                whatever.push((cur.0, next.1));
            } else {
                whatever.push(cur);
            }
        }
        if whatever.len() > 1 {
            println!("{:?}", whatever);
            return (whatever[0].1 + 1) * 4000000 + row_num;
        }
    }
    unreachable!()
}

// pub fn solve2() -> i64 {
//     // let data = sample();
//     // let max = 20;
//     let data = read_lines("/home/elliotbobrow/Downloads/day15.txt");
//     let max = 4000000;

//     let sensors: Vec<_> = data.into_iter().map(Sensor::from).collect();
//     let perims: HashSet<_> = sensors
//         .iter()
//         .flat_map(|sensor| {
//             let d = manhattan_dist(sensor.pos, sensor.beacon) + 1;
//             let mut perim = Vec::new();
//             for x in 0..=d {
//                 let new_x = sensor.pos.0 + x;
//                 let new_y = sensor.pos.1 + (d - x);
//                 if new_x >= 0 && new_x <= max && new_y >= 0 && new_y <= max {
//                     perim.push((new_x, new_y));
//                 }
//                 let new_x = sensor.pos.0 - x;
//                 let new_y = sensor.pos.1 - (d - x);
//                 if new_x >= 0 && new_x <= max && new_y >= 0 && new_y <= max {
//                     perim.push((new_x, new_y));
//                 }
//             }
//             perim
//         })
//         .collect();
//     for p in perims {
//         if !sensors.iter().any(|sensor| {
//             manhattan_dist(sensor.pos, p) <= manhattan_dist(sensor.pos, sensor.beacon)
//         }) {
//             println!("{:?}", p);
//             return (p.0 * 4000000) + p.1;12567351400528
//         }
//     }
//     unreachable!()
// }

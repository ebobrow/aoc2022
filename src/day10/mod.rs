use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#,
    )
}

fn inc_and_add(total_strengths: &mut i32, cycle: &mut i32, x: &i32) {
    *cycle += 1;
    if [20, 60, 100, 140, 180, 220].contains(cycle) {
        *total_strengths += (*cycle) * x;
    }
}

pub fn solve1() -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut total_strengths = 0;
    for line in read_lines("/home/elliotbobrow/Downloads/day10.txt") {
        if line == "noop" {
            inc_and_add(&mut total_strengths, &mut cycle, &x);
        } else {
            let (instr, amt) = line.split_once(' ').unwrap();
            assert_eq!(instr, "addx");
            inc_and_add(&mut total_strengths, &mut cycle, &x);
            inc_and_add(&mut total_strengths, &mut cycle, &x);
            x += amt.parse::<i32>().unwrap();
        }
    }
    total_strengths
}

fn inc_and_draw(cycle: &mut i32, x: &i32) {
    if (x - *cycle).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    *cycle += 1;
    if cycle == &40 {
        *cycle = 0;
        println!();
    }
}

pub fn solve2() {
    let mut cycle = 0;
    let mut x = 1;
    for line in read_lines("/home/elliotbobrow/Downloads/day10.txt") {
        // for line in sample() {
        if line == "noop" {
            inc_and_draw(&mut cycle, &x);
        } else {
            let (instr, amt) = line.split_once(' ').unwrap();
            assert_eq!(instr, "addx");
            inc_and_draw(&mut cycle, &x);
            inc_and_draw(&mut cycle, &x);
            x += amt.parse::<i32>().unwrap();
        }
    }
}

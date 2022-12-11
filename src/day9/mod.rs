use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
    )
}

fn mv(head: &mut (i32, i32), tail: &mut (i32, i32)) {
    let adjacent =
        |a: &(i32, i32), b: &(i32, i32)| (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1;
    if !adjacent(head, tail) {
        if head.0 == tail.0 {
            if head.1 > tail.1 {
                tail.1 += 1;
            } else {
                tail.1 -= 1;
            }
        } else if head.1 == tail.1 {
            if head.0 > tail.0 {
                tail.0 += 1;
            } else {
                tail.0 -= 1;
            }
        } else {
            if head.1 > tail.1 {
                tail.1 += 1;
            } else {
                tail.1 -= 1;
            }
            if head.0 > tail.0 {
                tail.0 += 1;
            } else {
                tail.0 -= 1;
            }
        }
    }
}

pub fn solve1() -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = vec![(0, 0)];
    let add_pts = |a: (i32, i32), b: (i32, i32)| (a.0 + b.0, a.1 + b.1);
    for line in read_lines("/home/elliotbobrow/Downloads/day9.txt") {
        let (dir, n) = line.split_once(' ').unwrap();
        let incr = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..n.parse().unwrap() {
            head = add_pts(head, incr);
            mv(&mut head, &mut tail);
            if !visited.contains(&tail) {
                visited.push(tail);
            }
        }
    }

    visited.len()
}

pub fn solve2() -> usize {
    let mut head = (0, 0);
    // let mut tails = vec![(0, 0); 10];
    let mut tail1 = (0, 0);
    let mut tail2 = (0, 0);
    let mut tail3 = (0, 0);
    let mut tail4 = (0, 0);
    let mut tail5 = (0, 0);
    let mut tail6 = (0, 0);
    let mut tail7 = (0, 0);
    let mut tail8 = (0, 0);
    let mut tail9 = (0, 0);
    let mut visited = vec![(0, 0)];
    let add_pts = |a: (i32, i32), b: (i32, i32)| (a.0 + b.0, a.1 + b.1);
    for line in read_lines("/home/elliotbobrow/Downloads/day9.txt") {
        let (dir, n) = line.split_once(' ').unwrap();
        let incr = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..n.parse().unwrap() {
            head = add_pts(head, incr);
            mv(&mut head, &mut tail1);
            mv(&mut tail1, &mut tail2);
            mv(&mut tail2, &mut tail3);
            mv(&mut tail3, &mut tail4);
            mv(&mut tail4, &mut tail5);
            mv(&mut tail5, &mut tail6);
            mv(&mut tail6, &mut tail7);
            mv(&mut tail7, &mut tail8);
            mv(&mut tail8, &mut tail9);
            if !visited.contains(&tail9) {
                visited.push(tail9);
            }
        }
    }

    visited.len()
}

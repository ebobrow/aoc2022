use crate::util::read_lines;

fn sample() -> Vec<&'static str> {
    r#"A Y
B X
C Z"#
        .split("\n")
        .collect()
}

pub fn solve1() -> u32 {
    read_lines("/home/elliotbobrow/Downloads/day2.txt")
        .iter()
        .map(|line| {
            let (a, b) = if let [a, b] = &line
                .split(' ')
                .map(|x| Into::<u32>::into(x.parse::<char>().unwrap()) - 64)
                .collect::<Vec<_>>()[..2]
            {
                (*a, b - 23)
            } else {
                unreachable!()
            };
            let score = if a == b {
                3
            } else if (b + 2) % 3 == (a % 3) {
                6
            } else {
                0
            };
            b + score
        })
        .sum()
}

pub fn solve2() -> u32 {
    read_lines("/home/elliotbobrow/Downloads/day2.txt")
        .iter()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let a: u32 = Into::<u32>::into(a.parse::<char>().unwrap()) - 64;
            match b {
                "X" => match a - 1 {
                    0 => 3,
                    n => n,
                },
                "Y" => 3 + a,
                "Z" => {
                    6 + match (a + 1) % 3 {
                        0 => 3,
                        n => n,
                    }
                }
                _ => unreachable!(),
            }
        })
        .sum()
}

use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
    )
}

#[derive(PartialEq)]
enum Pos {
    Cur,
    Target,
    Height(u32),
}

impl Pos {
    fn from(c: char) -> Self {
        match c {
            'S' => Self::Cur,
            'E' => Self::Target,
            _ => Self::Height(Into::<u32>::into(c) - 96),
        }
    }

    fn height(&self) -> u32 {
        match self {
            Pos::Cur => 1,
            Pos::Target => 26,
            Pos::Height(h) => *h,
        }
    }
}

fn path_from(map: &Vec<Vec<Pos>>, start: (usize, usize)) -> u32 {
    let mut explored = Vec::new();
    let mut frontier = vec![(start, 0)];
    while frontier.len() > 0 {
        let (cur_point, cur_cost) = frontier.remove(0);
        if map[cur_point.0][cur_point.1] == Pos::Target {
            return cur_cost;
        }
        explored.push(cur_point.clone());

        let mut adjacent = Vec::new();
        for inc in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new = (cur_point.0 as i32 + inc.0, cur_point.1 as i32 + inc.1);
            if new.0 < 0 || new.0 > map.len() as i32 - 1 {
                continue;
            }
            if new.1 < 0 || new.1 > map[0].len() as i32 - 1 {
                continue;
            }
            let new = (new.0 as usize, new.1 as usize);
            frontier = frontier
                .iter()
                .map(|(point, cost)| {
                    if *point == new && *cost > cur_cost + 1 {
                        (*point, cur_cost + 1)
                    } else {
                        (*point, *cost)
                    }
                })
                .collect();
            if explored.contains(&new) || frontier.iter().any(|(pt, _)| pt == &new) {
                continue;
            }
            if map[new.0][new.1].height() <= map[cur_point.0][cur_point.1].height() + 1 {
                adjacent.push((new, cur_cost + 1));
            }
        }
        frontier.extend(adjacent);
    }
    u32::MAX
}

pub fn solve1() -> u32 {
    let mut cur = (0, 0);
    let map: Vec<Vec<Pos>> = read_lines("/home/elliotbobrow/Downloads/day12.txt")
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        cur = (row, col);
                    }
                    Pos::from(c)
                })
                .collect()
        })
        .collect();
    path_from(&map, cur)
}

pub fn solve2() -> u32 {
    let map: Vec<Vec<Pos>> = read_lines("/home/elliotbobrow/Downloads/day12.txt")
        .iter()
        .map(|line| line.chars().map(Pos::from).collect())
        .collect();

    let mut aa = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == Pos::Height(1) {
                aa.push((row, col));
            }
        }
    }

    aa.into_iter().map(|a| path_from(&map, a)).min().unwrap()
}

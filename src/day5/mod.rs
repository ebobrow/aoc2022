use crate::util::read_lines;

fn sample() -> Vec<String> {
    r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
        .split('\n')
        .map(String::from)
        .collect()
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn from(lines: &[String]) -> Self {
        let mut lines: Vec<_> = lines[..lines.len() - 1]
            .iter()
            .map(|line| {
                let line_vec: Vec<_> = line.chars().collect();
                line_vec
                    .chunks(4)
                    .map(|c| {
                        *c.iter()
                            .find(|x| ![' ', '[', ']'].contains(x))
                            .unwrap_or(&' ')
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        lines.reverse();
        let mut stacks = vec![Vec::new(); lines[0].len()];
        for i in 0..lines[0].len() {
            for line in &lines {
                if line[i] != ' ' {
                    stacks[i].push(line[i]);
                }
            }
        }
        Stacks { stacks }
    }

    fn mv(&mut self, from: usize, to: usize) {
        let item = self.stacks[from - 1].pop().unwrap();
        self.stacks[to - 1].push(item);
    }

    fn mv_many(&mut self, n: usize, from: usize, to: usize) {
        let from = &mut self.stacks[from - 1];
        let mut items: Vec<_> = from.drain(from.len() - n..).collect();
        self.stacks[to - 1].append(&mut items);
    }

    fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

pub fn solve1() -> String {
    // let data = sample();
    let data = read_lines("/home/elliotbobrow/Downloads/day5.txt");
    let mut data = data.split(String::is_empty);
    let mut stacks = Stacks::from(data.next().unwrap());
    let instructions = data.next().unwrap();
    for instruction in instructions {
        let parts: Vec<_> = instruction
            .split(' ')
            .filter_map(|word| word.parse::<usize>().ok())
            .collect();
        for _ in 0..parts[0] {
            stacks.mv(parts[1], parts[2]);
        }
    }
    stacks.tops()
}

pub fn solve2() -> String {
    // let data = sample();
    let data = read_lines("/home/elliotbobrow/Downloads/day5.txt");
    let mut data = data.split(String::is_empty);
    let mut stacks = Stacks::from(data.next().unwrap());
    let instructions = data.next().unwrap();
    for instruction in instructions {
        let parts: Vec<_> = instruction
            .split(' ')
            .filter_map(|word| word.parse::<usize>().ok())
            .collect();
        stacks.mv_many(parts[0], parts[1], parts[2]);
    }
    stacks.tops()
}

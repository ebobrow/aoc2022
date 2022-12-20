use std::cmp::Ordering;

use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#,
    )
}

#[derive(PartialEq)]
enum Item {
    List(Vec<Item>),
    Integer(u32),
}

fn read_list(list: &str) -> Vec<Item> {
    if list == "[]" {
        return Vec::new();
    }
    let contents = &list[1..list.len() - 1];
    assert_eq!(&list[0..1], "[");
    assert_eq!(&list[list.len() - 1..], "]");
    let mut items = Vec::new();
    let mut cur_item = String::new();
    let mut open_bracket = None;
    let mut nesting_level = 0;
    for (i, c) in contents.chars().enumerate() {
        match c {
            ',' => {
                if open_bracket.is_none() && &contents[i - 1..i] != "]" {
                    items.push(Item::Integer(cur_item.parse().unwrap()));
                    cur_item = String::new();
                }
            }
            '[' => {
                if open_bracket.is_none() {
                    open_bracket = Some(i);
                    nesting_level += 1;
                } else {
                    nesting_level += 1;
                }
            }
            ']' => {
                nesting_level -= 1;
                if nesting_level == 0 {
                    items.push(Item::List(read_list(&contents[open_bracket.unwrap()..=i])));
                    open_bracket = None;
                }
            }
            _ if open_bracket.is_none() => cur_item.push(c),
            _ => {}
        }
    }
    if !cur_item.is_empty() {
        items.push(Item::Integer(cur_item.parse().unwrap()));
    }
    items
}

fn compare_lists(aa: &[Item], bb: &[Item]) -> Option<bool> {
    for pair in aa.iter().zip(bb) {
        match pair {
            (Item::Integer(a), Item::Integer(b)) => {
                if a < b {
                    return Some(true);
                } else if a > b {
                    return Some(false);
                }
            }
            (Item::List(a), Item::List(b)) => {
                if let Some(res) = compare_lists(a, b) {
                    return Some(res);
                }
            }
            (Item::Integer(a), Item::List(b)) => {
                if let Some(res) = compare_lists(&[Item::Integer(*a)], b) {
                    return Some(res);
                }
            }
            (Item::List(a), Item::Integer(b)) => {
                if let Some(res) = compare_lists(a, &[Item::Integer(*b)]) {
                    return Some(res);
                }
            }
        }
    }
    if aa.len() > bb.len() {
        return Some(false);
    } else if aa.len() < bb.len() {
        return Some(true);
    }
    None
}

pub fn solve1() -> usize {
    // sample()
    read_lines("/home/elliotbobrow/Downloads/day13.txt")
        .split(|line| line.is_empty())
        .enumerate()
        .filter(|(_, pair)| {
            if let [aa, bb] = pair {
                let [aa, bb] = [read_list(aa), read_list(bb)];
                compare_lists(&aa[..], &bb[..]).unwrap()
            } else {
                unreachable!()
            }
        })
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve2() -> usize {
    let mut lines: Vec<_> = 
        // sample()
        read_lines("/home/elliotbobrow/Downloads/day13.txt")
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| read_list(line))
        .collect();
    lines.extend([
        vec![Item::List(vec![Item::Integer(2)])],
        vec![Item::List(vec![Item::Integer(6)])],
    ]);
    lines.sort_by(|a, b| match compare_lists(a, b) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });
    let div1 = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line[..] == [Item::List(vec![Item::Integer(2)])])
        .unwrap()
        .0
        + 1;
    let div2 = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line[..] == [Item::List(vec![Item::Integer(6)])])
        .unwrap()
        .0
        + 1;

    div1 * div2
}

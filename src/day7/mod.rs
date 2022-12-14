use std::collections::HashMap;

use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
    )
}

#[derive(Debug)]
enum Item {
    Size(u64),
    Dir(String),
}

fn gen_fs() -> HashMap<String, u64> {
    let mut fs: HashMap<String, Vec<Item>> = HashMap::new();
    let mut cur = String::new();
    fs.insert(String::from("/"), Vec::new());
    // let data = sample();
    let data = read_lines("/home/elliotbobrow/Downloads/day7.txt");
    for line in data {
        if line.starts_with('$') {
            if line.len() > 4 && &line[2..4] == "cd" {
                match &line[5..] {
                    ".." => {
                        cur = fs
                            .iter()
                            .find(|(_, v)| {
                                v.iter()
                                    .any(|item| matches!(item, Item::Dir(dir) if dir == &cur))
                            })
                            .unwrap()
                            .0
                            .clone();
                    }
                    name => {
                        cur.push_str(name);
                    }
                }
            }
        } else if line.starts_with("dir") {
            fs.entry(cur.to_string())
                .and_modify(|items| items.push(Item::Dir(format!("{}{}", cur, &line[4..]))))
                .or_insert_with(|| vec![Item::Dir(format!("{}{}", cur, &line[4..]))]);
        } else {
            let (size, _) = line.split_once(' ').unwrap();
            fs.entry(cur.to_string())
                .and_modify(|items| items.push(Item::Size(size.parse().unwrap())))
                .or_insert_with(|| vec![Item::Size(size.parse().unwrap())]);
        }
    }

    let mut resolved = HashMap::new();
    loop {
        let mut done_something = false;
        for (k, v) in &fs {
            if resolved.contains_key(k) {
                continue;
            }
            if !v
                .iter()
                .any(|item| matches!(item, Item::Dir(name) if !resolved.contains_key(name)))
            {
                done_something = true;
                resolved.insert(
                    k.clone(),
                    v.iter()
                        .map(|item| match item {
                            Item::Size(s) => s,
                            Item::Dir(name) => resolved.get(name).unwrap(),
                        })
                        .sum(),
                );
            }
        }
        if !done_something {
            break;
        }
    }
    resolved
}

pub fn solve1() -> u64 {
    let resolved = gen_fs();

    resolved.values().filter(|size| size <= &&100000).sum()
}

pub fn solve2() -> u64 {
    let resolved = gen_fs();

    let used_space = resolved.get("/").unwrap();
    let free_space = 70000000 - used_space;
    let to_delete = 30000000 - free_space;
    *resolved
        .values()
        .filter(|size| size >= &&to_delete)
        .min()
        .unwrap()
}

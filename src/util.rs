use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap()
}

pub fn sample(raw: &str) -> Vec<&str> {
    raw.split('\n').collect()
}

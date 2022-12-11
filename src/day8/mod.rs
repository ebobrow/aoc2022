use crate::util::{self, read_lines};

fn sample() -> Vec<&'static str> {
    util::sample(
        r#"30373
25512
65332
33549
35390"#,
    )
}

fn visible(grid: Vec<Vec<char>>, r: usize, c: usize) -> bool {
    if (0..c).all(|col| grid[r][col] < grid[r][c]) {
        return true;
    }
    if (0..r).all(|row| grid[row][c] < grid[r][c]) {
        return true;
    }
    if (r + 1..grid.len()).all(|row| grid[row][c] < grid[r][c]) {
        return true;
    }
    if (c + 1..grid[0].len()).all(|col| grid[r][col] < grid[r][c]) {
        return true;
    }
    false
}

pub fn solve1() -> usize {
    // let data = sample();
    let data = read_lines("/home/elliotbobrow/Downloads/day8.txt");
    let grid: Vec<Vec<char>> = data.iter().map(|row| row.chars().collect()).collect();
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if visible(grid.clone(), r, c) {
                count += 1;
            }
        }
    }
    count
}

pub fn solve2() -> usize {
    // let data = sample();
    let data = read_lines("/home/elliotbobrow/Downloads/day8.txt");
    let grid: Vec<Vec<char>> = data.iter().map(|row| row.chars().collect()).collect();
    let mut max = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let left = match (0..c)
                .rev()
                .take_while(|&col| grid[r][col] < grid[r][c])
                .count()
            {
                n if n == c => n,
                n => n + 1,
            };
            let up = match (0..r)
                .rev()
                .take_while(|&row| grid[row][c] < grid[r][c])
                .count()
            {
                n if n == r => n,
                n => n + 1,
            };
            let down = match (r + 1..grid.len())
                .take_while(|&row| grid[row][c] < grid[r][c])
                .count()
            {
                n if n == grid.len() - (r + 1) => n,
                n => n + 1,
            };
            let right = match (c + 1..grid[0].len())
                .take_while(|&col| grid[r][col] < grid[r][c])
                .count()
            {
                n if n == grid[0].len() - (c + 1) => n,
                n => n + 1,
            };

            let score = left * up * down * right;
            if score > max {
                max = score;
            }
        }
    }
    max
}

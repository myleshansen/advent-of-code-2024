use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let levels = create_lists_from_file(Path::new("input.txt"));

    let safe_count = count_safe_levels(levels);

    println!("Safe level count: {}", safe_count);
}

fn create_lists_from_file(file_name: &Path) -> Vec<Vec<i32>> {
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut levels: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line_str = line.expect("Unable to read line");
        let line: Vec<i32> = line_str
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        levels.push(line);
    }

    return levels;
}

fn count_safe_levels(levels: Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for level in levels.iter() {
        if level[0] <= level[1] {
            safe_count += check_increasing(level);
        } else if level[0] >= level[1] {
            safe_count += check_decreasing(level);
        }
    }
    safe_count
}

fn check_decreasing(level: &Vec<i32>) -> i32 {
    for i in 0..level.len() - 1 {
        if level[i] <= level[i + 1] || level[i + 1] < level[i] - 3 {
            return 0;
        }
    }
    1
}

fn check_increasing(level: &Vec<i32>) -> i32 {
    for i in 0..level.len() - 1 {
        if level[i] >= level[i + 1] || level[i + 1] > level[i] + 3 {
            return 0;
        }
    }
    1
}

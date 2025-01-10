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
    for level in levels {
        let n = level.len();
        if level[0] > level[n - 1] {
            if check_valid_decreasing(&level) {
                safe_count += 1;
            } else {
                for idx in 0..level.len() {
                    let mut level_new = level.clone();
                    level_new.remove(idx);
                    if check_valid_decreasing(&level_new) {
                        safe_count += 1;
                        break;
                    }
                }
            }
        } else {
            if check_valid_increasing(&level) {
                safe_count += 1;
            } else {
                for idx in 0..level.len() {
                    let mut level_new = level.clone();
                    level_new.remove(idx);
                    if check_valid_increasing(&level_new) {
                        safe_count += 1;
                        break;
                    }
                }
            }
        }
    }
    safe_count
}

fn check_valid_decreasing(level: &Vec<i32>) -> bool {
    for i in 0..level.len() - 1 {
        if level[i] <= level[i + 1] || level[i + 1] < level[i] - 3 {
            return false;
        }
    }
    true
}

fn check_valid_increasing(level: &Vec<i32>) -> bool {
    for i in 0..level.len() - 1 {
        if level[i] >= level[i + 1] || level[i + 1] > level[i] + 3 {
            return false;
        }
    }
    true
}

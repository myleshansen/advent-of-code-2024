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
        let mut sorted_level = level.clone();
        sorted_level.sort();
        if check_valid(&sorted_level) {
            if level[0] > level[n - 1] {
                safe_count += check_decreasing(&level);
            } else {
                safe_count += check_increasing(&level);
            }
        }
    }
    safe_count
}

fn check_valid(level: &Vec<i32>) -> bool {
    let mut dampener = 0;
    for i in 0..level.len() - 1 {
        if level[i] >= level[i + 1] {
            dampener += 1;
            if dampener > 1 {
                return false;
            }
        }
    }

    true
}

fn check_decreasing(level: &Vec<i32>) -> i32 {
    let mut dampener = 0;
    let mut l = 0;
    let mut r = 1;

    while r < level.len() {
        if level[l] <= level[r] || level[r] < level[l] - 3 {
            dampener += 1;
            if dampener > 1 {
                return 0;
            }
            r += 1;
        } else {
            r += 1;
            l = r - 1;
        }
    }
    1
}

fn check_increasing(level: &Vec<i32>) -> i32 {
    let mut dampener = 0;
    let mut l = 0;
    let mut r = 1;

    while r < level.len() {
        if level[l] >= level[r] || level[r] > level[l] + 3 {
            dampener += 1;
            if dampener > 1 {
                return 0;
            }
            r += 1;
        } else {
            r += 1;
            l = r - 1;
        }
    }
    1
}

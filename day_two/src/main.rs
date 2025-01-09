use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let (vec1, vec2) = create_lists_from_file(Path::new("src/input.txt"));

    let final_score = create_similarity_map_score(vec1, vec2);

    println!("Total score: {}", final_score);
}

fn create_lists_from_file(file_name: &Path) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_str = line.expect("Unable to read line");
        let line: Vec<&str> = line_str.split_whitespace().collect();
        list_one.push(line[0].parse().expect("Failed to parse string to integer!"));
        list_two.push(line[1].parse().expect("Failed to parse string to integer!"));
    }

    return (list_one, list_two);
}

fn create_similarity_map_score(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut occurences = HashMap::new();

    for (_, val) in vec2.iter().enumerate() {
        *occurences.entry(val).or_insert(0) += 1;
    }

    calculate_similarity(occurences, vec1)
}

fn calculate_similarity(occurences: HashMap<&i32, i32>, vec1: Vec<i32>) -> i32 {
    let mut score = 0;
    for (_, val) in vec1.iter().enumerate() {
        if let Some(&count) = occurences.get(val) {
            score += count * val;
        }
    }
    score
}

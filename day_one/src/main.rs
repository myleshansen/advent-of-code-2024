use ::std::fs::File;
use ::std::path::Path;
use std::io::{BufRead, BufReader};

fn main() {
    let (vec1, vec2) = create_lists_from_file(Path::new("src/input.txt"));

    let total_distance = sort_and_calculate_distance(vec1, vec2);

    println!("Total distance: {}", total_distance)
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

fn sort_and_calculate_distance(mut vec1: Vec<i32>, mut vec2: Vec<i32>) -> i32 {
    vec1.sort();
    vec2.sort();

    let mut distance: i32 = 0;

    for (i, _) in vec1.iter().enumerate() {
        distance += (vec1[i] - vec2[i]).abs();
    }

    distance
}

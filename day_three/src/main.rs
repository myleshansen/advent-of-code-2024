use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mults = find_mults_regex_match(Path::new("input.txt"));

    let total = calculate_mults_sum(mults);
    println!("Total: {}", total);
}

fn find_mults_regex_match(file_name: &Path) -> Vec<Vec<String>> {
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut mults: Vec<Vec<String>> = Vec::new();

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Unable to read line"))
        .collect();
    for line_str in lines {
        // Find all matches
        let matches: Vec<String> = re
            .find_iter(&line_str)
            .map(|mat| mat.as_str().to_string())
            .collect();
        mults.push(matches);
    }

    mults
}

fn calculate_mults_sum(mults: Vec<Vec<String>>) -> i32 {
    let mut sum: i32 = 0;

    for mult in mults.iter() {
        for str in mult.iter() {
            let string_split = str[3..].split(',').collect::<Vec<&str>>();
            let num1 = string_split[0].replace("(", "");
            let num2 = string_split[1].replace(")", "");

            sum += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
        }
    }

    sum
}

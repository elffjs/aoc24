use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file =  File::open("inputs/day1")?;
    let reader = BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();


    for line in reader.lines() {
        let line = line?;

        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    let result1: i32 = left.iter().zip(right.iter()).map(|x| (x.0 - x.1).abs()).sum();

    println!("Total distance: {}", result1);

    let mut right_counts = HashMap::new();

    for x in right {
        *right_counts.entry(x).or_insert(0) += 1;
    }

    let result2: i32 = left.iter().map(|x| x * right_counts.get(x).copied().unwrap_or(0)).sum();

    println!("Similarity score: {}", result2);

    Ok(())
}

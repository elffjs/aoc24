use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day1() -> std::io::Result<()> {
    let file = File::open("inputs/day1")?;
    let reader = BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    let result1: u32 = left
        .iter()
        .zip(right.iter())
        .map(|x| x.0.abs_diff(*x.1))
        .sum();

    println!("Total distance: {}", result1);

    let mut right_counts = HashMap::new();

    for x in right {
        *right_counts.entry(x).or_insert(0) += 1;
    }

    let result2: i32 = left
        .iter()
        .map(|x| x * right_counts.get(x).copied().unwrap_or(0))
        .sum();

    println!("Similarity score: {}", result2);

    Ok(())
}

fn day2() -> std::io::Result<()> {
    let file = File::open("inputs/day2")?;
    let reader = BufReader::new(file);

    let mut safe_count: i32 = 0;
    let mut safe_with_dampener_count: i32 = 0;

    for line in reader.lines() {
        let line = line?;

        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        fn is_safe_without_dampener(levels: &Vec<i32>) -> bool {
            if levels.len() <= 1 {
                return true;
            }

            let differences: Vec<i32> = levels.windows(2).map(|x| x[1] - x[0]).collect();

            let first_difference = differences[0];

            if first_difference == 0 {
                return false;
            }

            let increasing = first_difference > 0;

            return differences
                .iter()
                .all(|&x| x.abs() >= 1 && x.abs() <= 3 && (x > 0) == increasing);
        }

        if is_safe_without_dampener(&levels) {
            safe_count += 1;
            safe_with_dampener_count += 1;
        } else {
            for i in 0..levels.len() {
                let mut v = levels.clone();
                v.remove(i);
                if is_safe_without_dampener(&v) {
                    safe_with_dampener_count += 1;
                    break;
                }
            }
        }
    }

    println!("Safe reports: {}", safe_count);
    println!("Safe reports with dampener: {}", safe_with_dampener_count);

    Ok(())
}

fn main() -> std::io::Result<()> {
    day2()
}

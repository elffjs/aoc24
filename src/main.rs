use regex::Regex;
use std::collections::HashMap;
use std::fs;
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

fn day3() -> std::io::Result<()> {
    let program = fs::read_to_string("inputs/day3")?;

    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();

    let dos: Vec<usize> = do_pattern
        .captures_iter(&program)
        .map(|m| m.get(0).unwrap().start())
        .collect();
    let donts: Vec<usize> = dont_pattern
        .captures_iter(&program)
        .map(|m| m.get(0).unwrap().start())
        .collect();

    let mut tot: i32 = 0;
    let mut enabled_tot: i32 = 0;

    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for m in pattern.captures_iter(&program) {
        let pos = m.get(0).unwrap().start();

        let prev_do = dos.iter().filter(|i| **i < pos).max();
        let prev_dont = donts.iter().filter(|i| **i < pos).max();

        let x: i32 = m[1].parse().unwrap();
        let y: i32 = m[2].parse().unwrap();

        let prod = x * y;

        tot += prod;

        if prev_dont < prev_do {
            enabled_tot += prod;
        }
    }

    println!("Sum of multiplications: {}", tot);
    println!("Sum of enabled multiplications: {}", enabled_tot);

    Ok(())
}

fn day4() -> std::io::Result<()> {
    let word = "XMAS";

    let file = File::open("inputs/day4")?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    let puzzle: Vec<&[u8]> = lines.iter().map(|l| l.as_bytes()).collect();

    let dirs = vec![
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
        [1, 1],
        [-1, 1],
        [1, -1],
        [-1, -1],
    ];

    let rows = puzzle.len() as i32;
    let cols = puzzle[0].len() as i32;

    let in_bounds =
        |pos: [i32; 2]| -> bool { (0..rows).contains(&pos[0]) && (0..cols).contains(&pos[1]) };

    let mut tot = 0;

    for line in lines.clone() {
        println!("{}", line);
    }

    let check_pos_dir = |pos: [i32; 2], dir: [i32; 2]| -> bool {
        let mut next = pos;
        for c in word.bytes() {
            if !in_bounds(next) || puzzle[next[0] as usize][next[1] as usize] != c {
                return false;
            }
            next = [next[0] + dir[0], next[1] + dir[1]];

        }
        println!("{:?} {:?} good", pos, dir);
        true
    };

    let mut check_pos = |pos: [i32; 2]| {
        for dir in dirs.clone() {
            if check_pos_dir(pos, dir) {
                tot += 1;
            }
        }
    };

    for row in 0..rows {
        for col in 0..cols {
            check_pos([row, col].clone());
        }
    }

    println!("XMAS count: {}", tot);

    // println!("{:?}", reader);
    Ok(())
}

fn main() -> std::io::Result<()> {
    day4()
}

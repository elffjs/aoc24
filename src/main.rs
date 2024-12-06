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

use std::ops::Add;

fn day4() -> std::io::Result<()> {
    let word = "XMAS";

    #[derive(Debug, Clone, Copy)]
    struct Pos(i32, i32);

    impl Add for Pos {
        type Output = Pos;

        fn add(self, right: Self) -> Self::Output {
            Pos(self.0 + right.0, self.1 + right.1)
        }
    }

    let file = File::open("inputs/day4")?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    let puzzle: Vec<&[u8]> = lines.iter().map(|l| l.as_bytes()).collect();

    let dirs: Vec<Pos> = (-1..=1)
        .flat_map(|r| {
            (-1..=1).flat_map(move |c| {
                if r == 0 && c == 0 {
                    None
                } else {
                    Some(Pos(r, c))
                }
            })
        })
        .collect();

    let rows = puzzle.len() as i32;
    let cols = puzzle[0].len() as i32;

    let in_bounds = |pos: Pos| -> bool { (0..rows).contains(&pos.0) && (0..cols).contains(&pos.1) };

    let mut tot = 0;

    let check_pos_dir = |pos: Pos, dir: Pos| -> bool {
        let mut next = pos;
        for c in word.bytes() {
            if !in_bounds(next) || puzzle[next.0 as usize][next.1 as usize] != c {
                return false;
            }
            next = next + dir;
        }
        true
    };

    let mut check_pos = |pos: Pos| {
        for dir in dirs.clone() {
            if check_pos_dir(pos, dir) {
                tot += 1;
            }
        }
    };

    for row in 0..rows {
        for col in 0..cols {
            check_pos(Pos(row, col));
        }
    }

    println!("XMAS count: {}", tot);

    let mut tot2 = 0;

    for (r, row) in puzzle.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            let rs = r as i32;
            let cs = c as i32;
            if *char == b'A'
                && in_bounds(Pos(rs - 1, cs - 1 as i32))
                && in_bounds(Pos(rs + 1 as i32, cs + 1 as i32))
            {
                let al = puzzle[r - 1][c - 1];
                let ar = puzzle[r - 1][c + 1];
                let br = puzzle[r + 1][c + 1];
                let bl = puzzle[r + 1][c - 1];

                if (al == b'M' && br == b'S' || al == b'S' && br == b'M')
                    && (bl == b'M' && ar == b'S' || bl == b'S' && ar == b'M')
                {
                    tot2 += 1;
                }
            }
        }
    }

    println!("X-MAS count: {}", tot2);

    Ok(())
}

use std::collections::HashSet;

fn day5() -> std::io::Result<()> {
    let file = File::open("inputs/day5")?;

    let mut order_mode = true;

    let mut before_after: HashSet<(i32, i32)> = HashSet::new();

    let mut tot = 0;
    let mut tot2 = 0;

    'line_loop: for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            println!("{:?}", before_after);
            order_mode = false;
            continue;
        }
        if order_mode {
            let (before, after) = line.split_once("|").unwrap();
            let (before, after): (i32, i32) = (before.parse().unwrap(), after.parse().unwrap());
            before_after.insert((before, after));
        } else {
            let row: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();

            let mut correct: Vec<i32> = Vec::new();

            println!("{:?}", row);

            let mut cared: HashSet<usize> = HashSet::new();

            for j in 0..row.len() {
                for i in 0..row.len() {
                    if cared.contains(&i) {
                        continue;
                    }
                    let mut good = true;
                    for j in 0..row.len() {
                        if cared.contains(&j) {
                            continue;
                        }
                        if before_after.contains(&(row[j], row[i])) {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        cared.insert(i);
                        correct.push(row[i]);
                    }
                }
            }

            println!("{:?}", correct);

            if row == correct {
                tot += row[row.len() / 2];
            } else {
                tot2 += correct[row.len() / 2];
            }
        }
    }

    println!("Sum of middle page numbers for correct rows: {}", tot);
    println!("Sum of middle page numbers for corrected rows: {}", tot2);

    Ok(())
}

fn main() -> std::io::Result<()> {
    day5()
}

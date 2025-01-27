use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_targets_and_factors() -> io::Result<Vec<(u64, Vec<u64>)>> {
    let path = "data/input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let targets_and_factors: Vec<(u64, Vec<u64>)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let numbers: Vec<&str> = line.split(':').collect();
            let target = numbers[0].parse::<u64>().unwrap();
            let factors: Vec<u64> = numbers[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            (target, factors)
        })
        .collect();

    Ok(targets_and_factors)
}

// Concatenate `lhs` and `rhs` as if they were strings of digits.
// For example, 4 || 5 => 45, 125 || 45 => 12545.
fn concat_nums(lhs: u64, rhs: u64) -> u64 {
    // Simple string-based concatenation
    let mut s = lhs.to_string();
    s.push_str(&rhs.to_string());
    s.parse().expect("Concatenation produced invalid number")
}

// Compute all results achievable by combining the given numbers in their given order,
// using `+`, `*`, or `||` (concatenation) strictly left to right.
fn compute_all_results_with_concat(nums: &[u64]) -> HashSet<u64> {
    let mut results = HashSet::new();
    let n = nums.len();

    if n == 0 {
        // No numbers => no results
        return results;
    }
    if n == 1 {
        // Only one number => that is the single result
        results.insert(nums[0]);
        return results;
    }

    // There are (n-1) "slots" and 3 operators => 3^(n-1) patterns.
    let max_op_patterns = 3u64.pow((n - 1) as u32);

    for ops_mask in 0..max_op_patterns {
        // We'll evaluate from left to right
        let mut value = nums[0];
        // We'll track which operator each slot uses
        for i in 1..n {
            // operator_index is in [0,1,2] => +, *, or ||
            let operator_index = (ops_mask / 3u64.pow(((i - 1) as u64).try_into().unwrap())) % 3;

            match operator_index {
                0 => {
                    // operator is '+'
                    value += nums[i];
                }
                1 => {
                    // operator is '*'
                    value *= nums[i];
                }
                2 => {
                    // operator is '||' (concatenation)
                    value = concat_nums(value, nums[i]);
                }
                _ => unreachable!(),
            }
        }
        results.insert(value);
    }

    results
}

fn main() -> io::Result<()> {
    let t1 = Instant::now();
    let targets_and_factors = read_targets_and_factors()?;

    let sum_targets = targets_and_factors
        .iter()
        .filter(|(target, factors)| {
            let results = compute_all_results_with_concat(factors);
            results.contains(target)
        })
        .map(|(target, _)| target)
        .sum::<u64>();

    let t2 = t1.elapsed();

    println!("Sum targets: {:?}", sum_targets);
    println!("Time elapsed: {:?}", t2);

    Ok(())
}

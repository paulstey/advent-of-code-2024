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

fn compute_all_results_fixed_order(nums: &[u64]) -> HashSet<u64> {
    let mut results = HashSet::new();

    // Handle the trivial case
    if nums.is_empty() {
        return results;
    }
    if nums.len() == 1 {
        results.insert(nums[0]);
        return results;
    }

    let n = nums.len();
    let max_op_patterns = 1 << (n - 1); // 2^(n-1) possible operator patterns

    // Bitmasking:
    //   We treat each possible “gap” between numbers as a bit in ops_mask.
    //   A 0 bit indicates we use addition (+), and a 1 bit indicates we use multiplication (*).
    //   let max_op_patterns = 1 << (n - 1); // 2^(n-1) possible operator patterns

    // Iterating Over All Patterns:
    //   For n numbers, there are n - 1 gaps. Hence, there are 2^(n−1) ways to combine + and *.
    //   We iterate ops_mask from 0 to 2^{n-1} - 1, and for each bit in ops_mask,
    //   we decide whether to apply + or *.
    for ops_mask in 0..max_op_patterns {
        // println!("ops_mask: {:b}", ops_mask);

        // Start with the first number
        let mut value = nums[0];

        // Apply operations left to right
        for i in 1..n {
            let use_mult = (ops_mask & (1 << (i - 1))) != 0;
            if use_mult {
                value *= nums[i];
            } else {
                value += nums[i];
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
            let results = compute_all_results_fixed_order(factors);
            results.contains(target)
        })
        .map(|(target, _)| target)
        .sum::<u64>();

    let t2 = t1.elapsed();

    println!("Sum targets: {:?}", sum_targets);
    println!("Time elapsed: {:?}", t2);

    Ok(())
}

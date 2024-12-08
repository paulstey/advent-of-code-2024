use anyhow::Result;
use regex::bytes::Regex;
use std::time::Instant;

fn process_corrupt_instructions() -> Result<usize> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;

    let result = re
        .captures_iter(include_bytes!("../data/input.txt"))
        .filter(|capture| {
            if capture.get(0).unwrap().as_bytes() == b"do()" {
                enabled = true;
                return false;
            } else if capture.get(0).unwrap().as_bytes() == b"don't()" {
                enabled = false;
            }
            enabled
        })
        .map(|capture| {
            atoi::atoi::<usize>(capture.get(2).unwrap().as_bytes()).unwrap()
                * atoi::atoi::<usize>(capture.get(3).unwrap().as_bytes()).unwrap()
        })
        .sum::<usize>() as usize;

    Ok(result)
}

fn main() {
    let start = Instant::now();
    let result = process_corrupt_instructions();
    match result {
        Ok(total) => println!("Total: {}", total),
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("Time: {:?}", start.elapsed());
}

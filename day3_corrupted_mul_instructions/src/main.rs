use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn process_corrupt_instructions() -> io::Result<i32> {
    let path = "data/input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let nums_re = Regex::new(r"\d{1,3},\d{1,3}").unwrap();
    for line in reader.lines() {
        let line = line?;

        for mul in mul_re.find_iter(&line) {
            // println!("Found a match: {}", mul.as_str());
            for nums in nums_re.find_iter(mul.as_str()) {
                let mut nums = nums.as_str().split(",");
                let num1 = nums.next().unwrap().parse::<i32>().unwrap();
                let num2 = nums.next().unwrap().parse::<i32>().unwrap();
                total += num1 * num2;
            }
        }
    }

    Ok(total)
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

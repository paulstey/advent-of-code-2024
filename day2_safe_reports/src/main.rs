use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn is_safe_report(row: &[i32]) -> bool {
    let mut differences = Vec::with_capacity(row.len() - 1);

    for i in 0..(row.len() - 1) {
        let abs_diff = (row[i] - row[i + 1]).abs();

        if row[i] == row[i + 1] {
            return false;
        } else if abs_diff > 3 {
            return false;
        } else {
            differences.push(row[i] - row[i + 1]);
        }
    }

    let all_positive = differences.iter().all(|&x| x > 0);
    let all_negative = differences.iter().all(|&x| x < 0);

    all_positive || all_negative
}

fn count_safe_reports() -> io::Result<i32> {
    let path = "data/input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut safe_reports = 0;

    for line in reader.lines() {
        let line = line?;

        let tokens: Vec<&str> = line.trim().split_whitespace().collect();

        let row: Vec<_> = tokens
            .into_iter()
            .map(|token| match token.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    panic!("Error: Failed to parse integer '{}'", token);
                }
            })
            .collect();

        safe_reports += is_safe_report(&row) as i32;
    }

    Ok(safe_reports)
}

fn main() {
    let t1 = Instant::now();
    let safe_reports = count_safe_reports().unwrap();

    let elapsed = t1.elapsed();
    println!("Number of safe reports: {:?}", safe_reports);

    println!("Elapsed Time: {:?}", elapsed);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn is_safe_report(row: &[i32]) -> bool {
    let mut differences = Vec::with_capacity(row.len());

    for i in 0..(row.len() - 1) {
        if (row[i] - row[i + 1]).abs() > 3 {
            return false;
        }
        differences.push(row[i] - row[i + 1]);
    }

    // In the case where all the differences are positive, we have a series with decreasing
    // values. In the case where all the differences are negative, we have a series with increasing
    // values. In both cases, the report is considered safe. If any differences are zero, the
    // report is considered unsafe, because we would have a repeated value.
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

        if is_safe_report(&row) {
            safe_reports += 1;
        } else {
            // Part 2:
            //   This is the section that removes one element from the row and allows us
            //   to test if the report is safe once the "bad" element is removed
            for i in 0..row.len() {
                let mut sub_row = row.clone();
                sub_row.remove(i);

                if is_safe_report(&sub_row) {
                    safe_reports += 1;
                    break;
                }
            }
        }
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

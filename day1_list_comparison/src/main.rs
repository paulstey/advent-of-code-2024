use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_two_columns() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let path = "data/input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let tokens: Vec<&str> = line.trim().split_whitespace().collect();

        let a = match tokens[0].parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Warning: Failed to parse integer '{}'", tokens[0]);
                continue;
            }
        };

        let b = match tokens[1].parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Warning: Failed to parse integer '{}'", tokens[1]);
                continue;
            }
        };

        column1.push(a);
        column2.push(b);
    }

    Ok((column1, column2))
}

fn main() {
    let t1 = Instant::now();
    let (mut column1, mut column2) = read_two_columns().unwrap();

    // The following two lines were only needed for Part 1 of today's problem.
    column1.sort();
    column2.sort();

    let similarity_score: i32 = column1
        .iter()
        .map(|&needle| needle * (column2.iter().filter(|&&elem| elem == needle).count() as i32))
        .sum();

    let elapsed = t1.elapsed();

    println!("Total difference: {}", similarity_score);
    println!("Elapsed time: {:?}", elapsed);
}

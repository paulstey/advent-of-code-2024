use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_two_columns() -> io::Result<(Vec<i32>, Vec<i32>)> {
    // Specify the path to the file
    let path = "data/input.txt";

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Create a buffered reader for efficient reading
    let reader = io::BufReader::new(file);

    // Initialize a vector to store pairs of integers
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let tokens: Vec<&str> = line.trim().split_whitespace().collect();

        // Parse the tokens into integers
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

    column1.sort();
    column2.sort();

    let mut total_difference = 0;

    for (a, b) in column1.iter().zip(column2.iter()) {
        total_difference += (a - b).abs();
    }

    let elapsed = t1.elapsed();

    println!("Total difference: {}", total_difference);
    println!("Elapsed time: {:?}", elapsed);
}

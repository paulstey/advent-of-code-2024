use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::time::Instant;

fn get_input_file() -> BufReader<File> {
    let f = File::open("data/input.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });
    BufReader::new(f)
}

fn main() {
    let t1 = Instant::now();

    let mut rules: HashSet<(i32, i32)> = HashSet::new();

    let mut lines = get_input_file().lines().map_while(Result::ok);

    // The first part of the input is the rules for the swapping.
    // The rules are in the format of "before|after" where before and after are integers.
    // The rules are read until an empty line is found.
    // Note that we're using `by_ref()` here in order to only partially consume the iterator.
    // This is because we want to use the iterator again to read the page updates.
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let mut line = line.trim().split('|');
        let before: i32 = line.next().unwrap().parse().unwrap();
        let after: i32 = line.next().unwrap().parse().unwrap();

        rules.insert((before, after));
    }

    // let mut part1 = 0;
    let mut part2 = 0;

    // The second part of the input is the pages updates we want to make.
    // The pages are in the format of a comma separated list of integers.
    // The pages are read until the end of the file.
    for line in lines {
        let mut page: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();

        // If the page updates are already sorted by the rules, we skip the line.
        // Otherwise, we sort the page by the rules and get the middle element.
        if page.is_sorted_by(|&a, &b| !rules.contains(&(b, a))) {
            // part1 += page[page.len() / 2];
            continue;
        } else {
            page.sort_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            part2 += page[page.len() / 2];
        }
    }
    let elapsed = t1.elapsed();

    println!("Elapsed: {:?}", elapsed);
    // println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

use ndarray::Array2;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::time::Instant;

fn read_char_grid() -> Result<Array2<char>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    let parsed_data: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let rows = parsed_data.len();
    let cols = if rows > 0 { parsed_data[0].len() } else { 0 };

    let flattened: Vec<char> = parsed_data.into_iter().flatten().collect();

    let array = Array2::from_shape_vec((rows, cols), flattened)
        .expect("Error constructing the Array2 from parsed data");

    Ok(array)
}

fn get_antenna_positions(antenna_grid: &Array2<char>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut positions = HashMap::new();

    let n_row = antenna_grid.nrows();
    let n_col = antenna_grid.ncols();

    for i in 0..n_row {
        for j in 0..n_col {
            if antenna_grid[(i, j)] != '.' {
                positions
                    .entry(antenna_grid[(i, j)])
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }

    positions
}

fn get_antinodes_positions(
    antenna_grid: &Array2<char>,
    antenna_positions: &HashMap<char, Vec<(usize, usize)>>,
) -> usize {
    let mut antinodes = HashSet::new();

    let n_row = antenna_grid.nrows();
    let n_col = antenna_grid.ncols();

    for (_, positions) in antenna_positions {
        for idx1 in 0..positions.len() {
            for idx2 in (idx1 + 1)..positions.len() {
                let a_i = 2 * positions[idx1].0 - positions[idx2].0;
                let a_j = 2 * positions[idx1].1 - positions[idx2].1;

                let b_i = 2 * positions[idx2].0 - positions[idx1].0;
                let b_j = 2 * positions[idx2].1 - positions[idx1].1;

                // NOTE: We don't need to check if `a_i >= 0` because the type of usize will always
                // be non-negative.
                if a_i < n_row && a_j < n_col {
                    antinodes.insert((a_i, a_j));
                }

                // NOTE: Just as above, we don't need to check if `b_i >= 0` because the type of usize
                // will always be non-negative.
                if b_i < n_row && b_j < n_col {
                    antinodes.insert((b_i, b_j));
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let t1 = Instant::now();

    let char_grid = read_char_grid().expect("Failed to read character grid");

    let antenna_positions = get_antenna_positions(&char_grid);

    let antinodes_count = get_antinodes_positions(&char_grid, &antenna_positions);

    let elapsed = t1.elapsed();

    println!("{:?}", antinodes_count);

    println!("Time taken: {:?}", elapsed);
}

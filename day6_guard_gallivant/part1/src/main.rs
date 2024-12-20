use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_floor_map() -> Result<Array2<char>> {
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

fn find_starting_position(floor_map: &Array2<char>) -> (usize, usize) {
    let (n_rows, n_cols) = floor_map.dim();

    for i in 0..n_rows {
        for j in 0..n_cols {
            if floor_map[[i, j]] == '^' {
                return (i, j);
            }
        }
    }

    panic!("Starting position not found");
}

fn walk(mut floor_map: Array2<char>, starting_position: (usize, usize)) -> Array2<char> {
    let (n_rows, n_cols) = floor_map.dim();

    let mut position = starting_position;
    let mut direction = Direction::Up;

    loop {
        let (i, j) = position;
        // println!("Position: {:?}, Direction: {:?}", position, direction);

        if i == 0 || i == n_rows - 1 || j == 0 || j == n_cols - 1 {
            floor_map[[i, j]] = 'X';

            return floor_map;
        }
        match direction {
            Direction::Up => {
                if floor_map[[i - 1, j]] == '#' {
                    direction = Direction::Right;
                } else {
                    position = (i - 1, j);
                    floor_map[[i, j]] = 'X';
                }
            }
            Direction::Down => {
                if floor_map[[i + 1, j]] == '#' {
                    direction = Direction::Left;
                } else {
                    position = (i + 1, j);
                    floor_map[[i, j]] = 'X';
                }
            }
            Direction::Left => {
                if floor_map[[i, j - 1]] == '#' {
                    direction = Direction::Up;
                } else {
                    position = (i, j - 1);
                    floor_map[[i, j]] = 'X';
                }
            }
            Direction::Right => {
                if floor_map[[i, j + 1]] == '#' {
                    direction = Direction::Down;
                } else {
                    position = (i, j + 1);
                    floor_map[[i, j]] = 'X';
                }
            }
        }
    }
}

fn main() {
    let t1 = Instant::now();

    let mut floor_map = read_floor_map().expect("Error reading the floor map");
    let starting_position = find_starting_position(&floor_map);
    floor_map[starting_position] = 'X';

    let walked_path = walk(floor_map, starting_position);

    let elapsed = t1.elapsed();

    println!(
        "Tiles visited: {:?}",
        walked_path
            .indexed_iter() // iterates over (idx, &val)
            .filter(|(_, &val)| val == 'X')
            .count()
    );
    println!("Time elapsed: {:?}", elapsed);
}

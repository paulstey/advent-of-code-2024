use ndarray::Array2;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::time::Instant;

const MAX_VISITS: i32 = 10;

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

fn walk(mut floor_map: Array2<char>, starting_position: (usize, usize)) -> i32 {
    let (n_rows, n_cols) = floor_map.dim();

    let mut position = starting_position;
    let mut direction = Direction::Up;
    let mut visited_tiles = HashMap::new();

    visited_tiles.insert(position, 1);

    loop {
        let (i, j) = position;
        // println!("Position: {:?}, Direction: {:?}", position, direction);

        let max_visits = visited_tiles.values().max().unwrap();

        if *max_visits > MAX_VISITS {
            return 1;
        }

        if i == 0 || i == n_rows - 1 || j == 0 || j == n_cols - 1 {
            floor_map[[i, j]] = 'X';

            return 0;
        }

        match direction {
            Direction::Up => {
                if floor_map[[i - 1, j]] == '#' {
                    direction = Direction::Right;
                } else {
                    position = (i - 1, j);

                    match visited_tiles.get(&position) {
                        Some(&count) => {
                            visited_tiles.insert(position, count + 1);
                        }
                        None => {
                            visited_tiles.insert(position, 1);
                        }
                    }
                    floor_map[[i, j]] = 'X';
                }
            }
            Direction::Down => {
                if floor_map[[i + 1, j]] == '#' {
                    direction = Direction::Left;
                } else {
                    position = (i + 1, j);

                    match visited_tiles.get(&position) {
                        Some(&count) => {
                            visited_tiles.insert(position, count + 1);
                        }
                        None => {
                            visited_tiles.insert(position, 1);
                        }
                    }
                    floor_map[[i, j]] = 'X';
                }
            }
            Direction::Left => {
                if floor_map[[i, j - 1]] == '#' {
                    direction = Direction::Up;
                } else {
                    position = (i, j - 1);

                    match visited_tiles.get(&position) {
                        Some(&count) => {
                            visited_tiles.insert(position, count + 1);
                        }
                        None => {
                            visited_tiles.insert(position, 1);
                        }
                    }
                    floor_map[[i, j]] = 'X';
                }
            }
            Direction::Right => {
                if floor_map[[i, j + 1]] == '#' {
                    direction = Direction::Down;
                } else {
                    position = (i, j + 1);

                    match visited_tiles.get(&position) {
                        Some(&count) => {
                            visited_tiles.insert(position, count + 1);
                        }
                        None => {
                            visited_tiles.insert(position, 1);
                        }
                    }
                    floor_map[[i, j]] = 'X';
                }
            }
        }
    }
}

fn main() {
    let t1 = Instant::now();

    let mut raw_floor_map = read_floor_map().expect("Error reading the floor map");
    let starting_position = find_starting_position(&raw_floor_map);
    raw_floor_map[starting_position] = 'X';

    let (n_rows, n_cols) = raw_floor_map.dim();
    let mut loops_created = 0;

    for i in 2..n_rows {
        for j in 0..n_cols {
            let mut floor_map = raw_floor_map.clone();

            if floor_map[[i, j]] == '.' && (i, j) != starting_position {
                floor_map[[i, j]] = '#';
                loops_created += walk(floor_map, starting_position);
            }
        }
    }

    let elapsed = t1.elapsed();

    println!("Loops created: {:?}", loops_created);
    println!("Time elapsed: {:?}", elapsed);
}

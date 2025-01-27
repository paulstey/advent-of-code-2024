use anyhow::Result;
use std::fs;
use std::time::Instant;

fn get_numbers() -> Result<Vec<i64>> {
    let numbers_string = fs::read_to_string("data/input.txt")?;

    let numbers_vec = numbers_string
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect::<Vec<i64>>();

    Ok(numbers_vec)
}

fn get_block_ids(numbers: &[i64]) -> Vec<i64> {
    let mut block_ids = Vec::new();
    let mut is_free = false;
    let mut id = 0;

    for n_steps in numbers {
        if is_free {
            let mut new_blocks = vec![-1; *n_steps as usize];
            block_ids.append(&mut new_blocks);
        } else {
            let mut new_blocks = vec![id; *n_steps as usize];

            block_ids.append(&mut new_blocks);

            id += 1;
        }

        is_free = !is_free;
    }

    block_ids
}

fn move_file_blocks(mut block_ids: Vec<i64>) -> Vec<i64> {
    let mut left_idx = 0;
    let mut right_idx = block_ids.len() - 1;

    while left_idx < right_idx {
        if block_ids[left_idx] == -1 {
            while block_ids[right_idx] < 0 {
                right_idx -= 1;
            }

            block_ids.swap(left_idx, right_idx);
            right_idx -= 1;
        } else {
            left_idx += 1;
        }
    }

    block_ids.into_iter().filter(|&x| x != -1).collect()
}

fn compute_checksum(block_ids: &[i64]) -> i64 {
    block_ids
        .iter()
        .enumerate()
        .map(|(i, &id)| i as i64 * id)
        .sum()
}

fn main() -> Result<()> {
    let start = Instant::now();

    let numbers = get_numbers()?;

    let block_ids = get_block_ids(&numbers);
    let compacted_ids = move_file_blocks(block_ids);
    let checksum = compute_checksum(&compacted_ids);
    let elapsed = start.elapsed();

    println!("File contents:\n{:?}", compacted_ids);
    println!("File contents:\n{:?}", checksum);
    println!("Elapsed time: {:?}", elapsed);

    Ok(())
}

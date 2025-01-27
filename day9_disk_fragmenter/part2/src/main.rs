use anyhow::Result;
use std::fs;
use std::time::Instant;


fn get_numbers() -> Result<Vec<i64>> {
    let numbers_string = fs::read_to_string("data/example1_input.txt")?;

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

fn find_next_gap(block_ids: &[i64], start_idx: usize, n_blocks: usize) -> Option<(usize, usize)> {
    if start_idx >= block_ids.len() {
        return None; // Base case: if start_idx is out of bounds, return None
    }

    let start_idx = match block_ids[start_idx..].iter().position(|&x| x == -1) {
        Some(pos) => start_idx + pos,
        None => return None, // No more gaps found
    };

    let mut end_idx = start_idx;

    while end_idx < block_ids.len() && block_ids[end_idx] == -1 {
        end_idx += 1;
    }

    let end_idx = end_idx as isize - 1;

    // If the gap is too small, search for the next gap
    if end_idx - start_idx as isize + 1 < n_blocks as isize {
        let next_start_idx = (end_idx + 1) as usize;
        return find_next_gap(block_ids, next_start_idx, n_blocks);
    } else {
        let end_idx = start_idx + n_blocks - 1;
        return Some((start_idx, end_idx));
    }
}

fn find_next_blocks_to_move(block_ids: &[i64], id: i64) -> (usize, usize) {
    let end_block_idx = block_ids.iter().rposition(|&x| x != -1 && x == id).unwrap();
    let mut start_block_idx = end_block_idx;

    let block_id = block_ids[end_block_idx]; // This is the actual value of the block ID we may
                                             // move depending on the gap size and the block size

    while block_ids[start_block_idx] == block_id {
        start_block_idx -= 1;
    }

    start_block_idx += 1; // Need to correct for the last increment in the
                          // while loop above which would have overstepped the
                          // bounds by one towards the left

    (start_block_idx, end_block_idx)
}

fn move_file_blocks(mut block_ids: Vec<i64>) -> Vec<i64> {
    let mut end_block_idx: usize = block_ids.len() - 1;
    let mut block_id = block_ids.iter().max().unwrap().clone();
    let mut gap_size = 0;
    let mut end_gap_idx: usize = 0;
    let mut start_block_idx: usize;
    let mut start_gap_idx: usize = 0;

    let mut n_iter = 0;

    while end_gap_idx < end_block_idx && n_iter < 1000 {
        println!("{:?}", n_iter);

        n_iter += 1;

        println!("Searching for blocks to move for block_id: {}", block_id);

        let (start_block_idx, end_block_idx) = find_next_blocks_to_move(&block_ids, block_id);

        println!(
            "Moving blocks from index range: {}..{}",
            start_block_idx, end_block_idx
        );

        let mut file_blocks_to_move = block_ids[start_block_idx..=end_block_idx].to_vec();
        let n_blocks = file_blocks_to_move.len();

        // NOTE: Instead of having `left_idx` to use with the `right_idx`, we use the `end_gap_idx` to
        // represent the left-most index of the gap currently available for moving blocks
        if let Some((start, end)) = find_next_gap(&block_ids, start_gap_idx, n_blocks) {
            (start_gap_idx, end_gap_idx) = (start, end);
            gap_size = end_gap_idx - start_gap_idx + 1;

            if start_gap_idx > start_block_idx {
                println!(
                    "Cannot swap file block from {} to empty gap at indices: {}..{}",
                    start_block_idx, start_gap_idx, end_gap_idx
                );
                start_gap_idx = 0;
                end_gap_idx = 0;

                continue;
            }
            println!("Swapping blocks: {:?}", file_blocks_to_move);

            file_blocks_to_move.swap_with_slice(&mut block_ids[start_gap_idx..=end_gap_idx]);

            for i in start_block_idx..=end_block_idx {
                block_ids[i] = -1;
            }

            println!("{:?}", block_ids);

            block_id -= 1;
            start_gap_idx = 0;
            end_gap_idx = 0;

            debug_assert!(block_id >= 0);
        } else {
            block_id -= 1;
            start_gap_idx = 0;
            end_gap_idx = 0;
        }
    }

    block_ids
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
    let compacted_ids = move_file_blocks(block_ids.clone());
    // let checksum = compute_checksum(&compacted_ids);
    let elapsed = start.elapsed();

    println!("file contents:\n{:?}", block_ids);

    println!("file contents:\n{:?}", compacted_ids);
    // println!("File contents:\n{:?}", checksum);
    println!("Elapsed time: {:?}", elapsed);

    Ok(())
}

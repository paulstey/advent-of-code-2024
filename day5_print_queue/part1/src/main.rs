use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_input_data() -> io::Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)> {
    let path = "data/input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut update_lists = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.contains('|') {
            // This first section is where we handle the ordering rules. If the line contains a
            // '|', then we know that the first number is the leader and the second number is the
            // succesor.
            let mut nums_iter = line.split('|');
            let leader: i32 = nums_iter.next().unwrap().parse().unwrap();
            let succesor: i32 = nums_iter.next().unwrap().parse().unwrap();

            if ordering_rules.contains_key(&leader) {
                ordering_rules.get_mut(&leader).unwrap().push(succesor);
            } else {
                ordering_rules.insert(leader, vec![succesor]);
            }
        } else if line.contains(',') {
            // This section specifies the update lists. These update list can be valid or invalid.
            // If page numbers listed in the update list complying with the ordering rules, then
            // the update list is valid. Otherwise, the update list is invalid.
            let update_list = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();

            update_lists.push(update_list);
        }
    }
    Ok((ordering_rules, update_lists))
}

fn is_update_list_valid(ordering_rules: &HashMap<i32, Vec<i32>>, update_list: &Vec<i32>) -> bool {
    // This function checks if the update list is valid or not. If the update list is valid, then
    // the function returns true. Otherwise, the function returns false.
    for i in 0..update_list.len() - 1 {
        for j in i + 1..update_list.len() {
            let before = update_list[i];
            let after = update_list[j];

            if ordering_rules.contains_key(&after) {
                if ordering_rules.get(&after).unwrap().contains(&before) {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let t1 = Instant::now();
    let (ordering_rules, update_lists) = read_input_data().unwrap();

    let sum_mid_values: i32 = update_lists
        .into_iter()
        .map(|update_list| {
            let is_valid = is_update_list_valid(&ordering_rules, &update_list);

            let increment_by = if is_valid {
                let mid_idx = update_list.len() / 2;
                update_list[mid_idx]
            } else {
                0
            };

            increment_by
        })
        .sum();

    let t2 = Instant::now();

    println!("Sum mid values: {:?}", sum_mid_values);
    println!("Total walltime: {:?}", t2.duration_since(t1));
}

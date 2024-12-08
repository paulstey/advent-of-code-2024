use std::time::Instant;

fn count_xmas_instances() -> usize {
    let input_mat = include_bytes!("../data/input.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    let mut cross = [0; 4];

    (0..input_mat[0].len() as isize)
        .flat_map(|i| (0..input_mat.len() as isize).map(move |j| (i, j)))
        .map(|(i, j)| {
            [
                (i + 1, j + 1), // Center
                (i, j),         // NE
                (i, j + 2),     // SE
                (i + 2, j),     // NW
                (i + 2, j + 2), // SW
            ]
        })
        .filter(|coords| {
            let mut iter = coords.iter().map(|(x, y)| {
                input_mat
                    .get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap_or_default()
            });

            if iter.next().is_none_or(|n| n != b'A') {
                return false;
            }

            cross[0] = iter.next().unwrap_or_default();
            cross[1] = iter.next().unwrap_or_default();
            cross[2] = iter.next().unwrap_or_default();
            cross[3] = iter.next().unwrap_or_default();

            &cross == b"MMSS" || &cross == b"MSMS" || &cross == b"SSMM" || &cross == b"SMSM"
        })
        .count()
}

fn main() {
    let start = Instant::now();

    let result = count_xmas_instances();

    println!("Result: {}", result);
    println!("Time: {:?}", start.elapsed());
}

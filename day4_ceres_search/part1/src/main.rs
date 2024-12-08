use std::time::Instant;

fn count_xmas_instances() -> usize {
    let mut word = [0; 4];
    let input_mat = include_bytes!("../data/input.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    (0..input_mat[0].len() as isize)
        .flat_map(|i| (0..input_mat.len() as isize).map(move |j| (i, j)))
        .flat_map(|(i, j)| {
            let coords_mat = [
                [(i, j), (i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)], // NE
                [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)],             // E
                [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)], // SE
                [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)],             // S
            ];

            coords_mat
        })
        .filter(|coords| {
            let mut iter = coords.iter().map(|(i, j)| {
                input_mat
                    .get(*j as usize)
                    .and_then(|row| row.get(*i as usize).copied())
                    .unwrap_or_default()
            });

            word.fill_with(|| iter.next().unwrap_or_default());

            &word == b"XMAS" || &word == b"SAMX"
        })
        .count()
}

fn main() {
    let start = Instant::now();

    let result = count_xmas_instances();

    println!("Result: {}", result);
    println!("Time: {:?}", start.elapsed());
}

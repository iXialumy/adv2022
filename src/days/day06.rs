use std::collections::HashSet;

pub fn day06() {
    let input = include_str!("../../input/06.txt");
    day06a(input);
    day06b(input);
}

fn day06a(input: &str) {
    let first = solve_for_len(input, 4);
    println!("Day06a: {first}");
}

fn day06b(input: &str) {
    let first = solve_for_len(input, 14);
    println!("Day06a: {first}");
}

fn solve_for_len(input: &str, len: usize) -> usize {
    let bytes: Vec<_> = input.bytes().collect();
    bytes
        .windows(len)
        .enumerate()
        .find(|(_index, slice)| all_unique(slice))
        .map(|(index, _)| index + len)
        .unwrap()
}

fn all_unique(slice: &&[u8]) -> bool {
    let hs: HashSet<_> = slice.iter().collect();
    hs.len() == slice.len()
}

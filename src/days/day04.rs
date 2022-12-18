use std::cmp::Ordering;

pub fn day04() {
    let input = include_str!("../../input/04.txt");
    day04a(input);
    day04b(input);
}

fn day04a(input: &str) {
    let contained = input
        .lines()
        .filter_map(parse_line)
        .filter(fully_contained)
        .count();

    println!("Day04a: {contained}");
}

fn day04b(input: &str) {
    let overlapping = input
        .lines()
        .filter_map(parse_line)
        .filter(overlap)
        .count();

    println!("Day04b: {overlapping}")
}

fn parse_line(line: &str) -> Option<(u32, u32, u32, u32)> {
    let (elf_a, elf_b) = line.split_once(',')?;
    let (from_a_str, to_a_str) = elf_a.split_once('-')?;
    let (from_b_str, to_b_str) = elf_b.split_once('-')?;

    Some((
        from_a_str.parse().ok()?,
        to_a_str.parse().ok()?,
        from_b_str.parse().ok()?,
        to_b_str.parse().ok()?,
    ))
}

fn fully_contained((a, b, c, d): &(u32, u32, u32, u32)) -> bool {
    match a.cmp(c) {
        Ordering::Less => { b >= d }
        Ordering::Equal => { true }
        Ordering::Greater => { b <= d }
    }
}

fn overlap((a, b, c, d): &(u32, u32, u32, u32)) -> bool {
    (c..=d).contains(&a) || (a..=b).contains(&c)
}
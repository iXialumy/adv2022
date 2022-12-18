use itertools::Itertools;

pub fn day01() {
    let input = include_str!("../../input/01.txt");
    day01a(input);
    day01b(input);
}

fn day01a(input: &str) {
    let max: u32 = input
        .split("\n\n")
        .map(to_calories)
        .map(|x| x.iter().sum())
        .max()
        .unwrap();

    println!("Day01a: {max}");
}
fn day01b(input: &str) {
    let sum: u32 = input
        .split("\n\n")
        .map(to_calories)
        .map(|x| x.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("Day01b: {sum}");
}

fn to_calories(lines: &str) -> Vec<u32> {
    lines.lines().filter_map(|x| x.parse().ok()).collect()
}

use std::collections::HashSet;

use itertools::Itertools;

pub fn day03() {
    let input = include_str!("../../input/03.txt");
    day03a(input);
    day03b(input);
}

fn day03a(input: &str) {
    let sum: u32 = input
        .lines()
        .map(rucksack_from_string)
        .flat_map(common_chars_a)
        .map(priority)
        .sum();

    println!("Day03a: {sum}")
}

fn day03b(input: &str) {
    let sum: u32 = input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(common_chars_b)
        .map(priority)
        .sum();
    println!("Day03b: {sum}");
}

fn rucksack_from_string(rucksack: &str) -> (&str, &str) {
    let len = rucksack.len();
    if len % 2 != 0 {
        panic!("Uneven Backpack found: {rucksack}");
    }
    rucksack.split_at(len / 2)
}

fn common_chars_a<'a>((str_a, str_b): (&'a str, &'a str)) -> HashSet<char> {
    let set_b: HashSet<_> = str_b.chars().collect();
    str_a.chars().filter(|c| set_b.contains(c)).collect()
}

fn common_chars_b<'a, I>(rucksacks: I) -> HashSet<char>
    where
        I: Iterator<Item=&'a str>,
{
    let mut acc: Vec<_> = ('a'..='z').chain('A'..='Z').collect();
    for rucksack in rucksacks {
        acc.retain(|c| rucksack.contains(*c));
    }
    acc.iter().cloned().collect()
}

fn priority(c: char) -> u32 {
    if !c.is_ascii_alphabetic() {
        panic!("Invalid Item found in Backpack: {c}");
    }
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

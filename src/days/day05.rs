use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn day05() {
    let input = include_str!("../../input/05.txt");

    day05a(input);
    day05b(input);
}

struct Towers(Vec<Vec<char>>);

impl std::fmt::Debug for Towers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_towers = self.0.len();
        let longest = self.0.iter().map(|tower| tower.len()).max().unwrap_or(0);

        writeln!(f)?;

        for j in (0..longest).rev() {
            for i in 0..num_towers {
                match self.0[i].get(j) {
                    Some(c) => f.write_fmt(format_args!("[{c}] "))?,
                    None => f.write_str("    ")?,
                };
            }
            writeln!(f)?;
        }
        f.write_str(&"-".repeat(num_towers * 4 - 1))?;
        Ok(())
    }
}

fn day05a(input: &str) {
    let (towers, instructions) = input
        .split_once("\n\n")
        .expect("Input for tower and instructions is not separated by empty line");

    let initial = parse_towers_setup(towers);
    let end = instructions
        .lines()
        .filter_map(parse_instruction)
        .fold(initial, |mut acc, x| {
            acc.perform_move_a(x);
            acc
        });
    let tops = end.tops();
    println!("Day05a: {tops}");
}

fn day05b(input: &str) {
    let (towers, instructions) = input
        .split_once("\n\n")
        .expect("Input for tower and instructions is not separated by empty line");

    let initial = parse_towers_setup(towers);
    let end = instructions
        .lines()
        .filter_map(parse_instruction)
        .fold(initial, |mut acc, x| {
            acc.perform_move_b(x);
            acc
        });
    let tops = end.tops();
    println!("Day05b: {tops}");
}

fn parse_towers_setup(towers: &str) -> Towers {
    let lines: Vec<_> = towers.lines().rev().collect();
    let numberings = lines[0];
    let num_towers = (numberings.len()) / 4 + 1;
    let mut towers = vec![Vec::new(); num_towers];

    for line in lines.iter().skip(1) {
        for (i, cr) in (&line.chars().chunks(4)).into_iter().enumerate() {
            let cr: String = cr.collect();
            let crate_char = cr.chars().nth(1).unwrap();
            if crate_char.is_whitespace() {
                continue;
            }
            towers[i].push(crate_char);
        }
    }

    Towers(towers)
}

fn parse_instruction(instruction: &str) -> Option<(usize, usize, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    let captures = RE.captures(instruction)?;
    Some((
        captures[1].parse::<usize>().ok()?,
        captures[2].parse::<usize>().ok()? - 1usize,
        captures[3].parse::<usize>().ok()? - 1usize,
    ))
}

impl Towers {
    fn perform_move_a(&mut self, (amount, from, to): (usize, usize, usize)) {
        let top = self.0[from].len();
        let range = (top - amount)..;
        let mut buffer: Vec<_> = self.0[from].drain(range).rev().collect();
        self.0[to].append(&mut buffer);
    }

    fn perform_move_b(&mut self, (amount, from, to): (usize, usize, usize)) {
        let top = self.0[from].len();
        let range = (top - amount)..;
        let mut buffer: Vec<_> = self.0[from].drain(range).collect();
        self.0[to].append(&mut buffer);
    }

    fn tops(&self) -> String {
        self.0
            .iter()
            .map(|tower| tower.last().unwrap_or(&' '))
            .filter(|c| c.is_alphabetic())
            .join("")
    }
}

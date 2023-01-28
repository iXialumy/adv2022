use std::iter;
use itertools::Itertools;

pub fn day10() {
    let input = include_str!("../../input/10.txt");
    day10a(input);
    day10b(input);
}

fn day10a(input: &str) {
    let sum: i32 = cycles(input)
        .filter(|(cycle, _)| cycle % 40 == 20)
        .map(|(cycle, x)| cycle * x)
        .sum();
    println!("Day10a: {sum}")
}

fn day10b(input: &str) {
    let mut iter = iter::once((0, 0)).chain(cycles(input));
    iter.next_back();

    let output = iter
        .chunks(40)
        .into_iter()
        .map(|chunk|
            chunk
                .map(render_pixel)
                .collect::<String>()
        )
        .join("\n");
    println!("Day10b:\n{output}")
}

fn cycles(input: &str) -> impl DoubleEndedIterator<Item=(i32, i32)> + '_ {
    input
        .lines()
        .flat_map(parse_command)
        .scan((1, 1), |acc, new| {
            *acc = (acc.0 + 1, acc.1 + new);
            Some(*acc)
        })
        .collect::<Vec<_>>()
        .into_iter()
}

fn parse_command(line: &str) -> Vec<i32> {
    if line.starts_with("noop") {
        vec![0]
    } else if line.starts_with("addx ") {
        let to_add: i32 = line.split_once(' ').unwrap().1.parse().unwrap();
        vec![0, to_add]
    } else {
        panic!()
    }
}

fn render_pixel((cycle, x): (i32, i32)) -> char {
    let difference = ((x) - ((cycle - 1) % 40)).abs();
    if difference < 2 { '█' } else { ' ' }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_pixel() {
        assert_eq!('█', render_pixel((1, 1)));
        assert_eq!('█', render_pixel((2, 1)));
        assert_eq!('█', render_pixel((3, 1)));
        assert_eq!(' ', render_pixel((4, 1)));
    }
}
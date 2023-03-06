use anyhow::anyhow;
use itertools::{repeat_n, Itertools};

#[allow(unused)]
pub fn day09() {
    // let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    let input = include_str!("../../input/09.txt");
    day09a(input);
    day09b(input);
}

fn day09a(input: &str) {
    let positions = head_positions(input)
        .scan((0, 0), |tail_pos, head_pos| {
            *tail_pos = new_tail_pos(tail_pos, head_pos);
            Some(*tail_pos)
        })
        .unique()
        .count();

    println!("Day09a: {positions:?}")
}

fn day09b(input: &str) {
    let head_positions = head_positions(input).collect_vec();
    let positions = (0..9).fold(head_positions, |mut acc, _| {
        // println!("{acc:?}\n");
        acc = acc
            .iter()
            .cloned()
            .scan((0, 0), |tail_pos, head_pos| {
                *tail_pos = new_tail_pos(tail_pos, head_pos);
                Some(*tail_pos)
            })
            .collect_vec();

        acc
    });
    // println!("{positions:?}");
    let count = positions.iter().unique().count();
    println!("Day09b: {count}");
}

fn new_tail_pos(tail_pos: &(i32, i32), head_pos: (i32, i32)) -> (i32, i32) {
    let tail_pos = *tail_pos;
    let offset_height = head_pos.0 - tail_pos.0;
    let offset_width = head_pos.1 - tail_pos.1;
    match (offset_height, offset_width) {
        (0, 0) => tail_pos,

        (1, 0) => tail_pos,
        (0, 1) => tail_pos,
        (-1, 0) => tail_pos,
        (0, -1) => tail_pos,
        (1, 1) => tail_pos,
        (-1, 1) => tail_pos,
        (1, -1) => tail_pos,
        (-1, -1) => tail_pos,

        (2, 0) => (tail_pos.0 + 1, tail_pos.1),
        (2, 1) => (tail_pos.0 + 1, tail_pos.1 + 1),
        (2, -1) => (tail_pos.0 + 1, tail_pos.1 - 1),

        (-2, 0) => (tail_pos.0 - 1, tail_pos.1),
        (-2, 1) => (tail_pos.0 - 1, tail_pos.1 + 1),
        (-2, -1) => (tail_pos.0 - 1, tail_pos.1 - 1),

        (0, 2) => (tail_pos.0, tail_pos.1 + 1),
        (1, 2) => (tail_pos.0 + 1, tail_pos.1 + 1),
        (-1, 2) => (tail_pos.0 - 1, tail_pos.1 + 1),

        (0, -2) => (tail_pos.0, tail_pos.1 - 1),
        (1, -2) => (tail_pos.0 + 1, tail_pos.1 - 1),
        (-1, -2) => (tail_pos.0 - 1, tail_pos.1 - 1),

        (2, 2) => (tail_pos.0 + 1, tail_pos.1 + 1),
        (-2, 2) => (tail_pos.0 - 1, tail_pos.1 + 1),
        (2, -2) => (tail_pos.0 + 1, tail_pos.1 - 1),
        (-2, -2) => (tail_pos.0 - 1, tail_pos.1 - 1),

        _ => unreachable!("Difference too large. Head: {head_pos:?}, Tail: {tail_pos:?}"),
    }
}

fn head_positions(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input
        .lines()
        .map(|line| parse_line(line).unwrap())
        .flat_map(expand_command)
        .scan((0, 0), |state, direction| {
            match direction {
                Direction::U => state.0 += 1,
                Direction::D => state.0 -= 1,
                Direction::R => state.1 += 1,
                Direction::L => state.1 -= 1,
            };

            Some(state.to_owned())
        })
}

#[derive(Debug, Eq, PartialEq)]
struct Command {
    direction: Direction,
    length: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    R,
    U,
    L,
    D,
}

fn parse_line(line: &str) -> anyhow::Result<Command> {
    let (direction, length) = line
        .split_once(' ')
        .ok_or(anyhow!("Malformated Command: {line}"))?;
    let direction = match direction {
        "U" => Direction::U,
        "D" => Direction::D,
        "L" => Direction::L,
        "R" => Direction::R,
        _ => return Err(anyhow!("Cannot Parse Direction: {direction}")),
    };
    let length = length.parse()?;
    Ok(Command { direction, length })
}

fn expand_command(command: Command) -> impl Iterator<Item = Direction> {
    repeat_n(command.direction, command.length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_success() {
        let command = parse_line("U 5").unwrap();
        assert_eq!(command.direction, Direction::U);
        assert_eq!(command.length, 5);

        let command = parse_line("R 10").unwrap();
        assert_eq!(command.direction, Direction::R);
        assert_eq!(command.length, 10);

        let command = parse_line("L 2").unwrap();
        assert_eq!(command.direction, Direction::L);
        assert_eq!(command.length, 2);

        let command = parse_line("D 7").unwrap();
        assert_eq!(command.direction, Direction::D);
        assert_eq!(command.length, 7);
    }

    #[test]
    fn test_parse_line_malformated_error() {
        let result = parse_line("U");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Malformated Command: U");
    }

    #[test]
    fn test_parse_line_cannot_parse_direction_error() {
        let result = parse_line("X 5");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot Parse Direction: X");

        let result = parse_line("5 U");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot Parse Direction: 5");
    }

    #[test]
    fn test_parse_line_parse_length_error() {
        let result = parse_line("U five");
        assert!(result.is_err());
    }

    #[test]
    fn test_head_positions() {
        let input = "U 2\nR 3\nD 1";
        let expected = vec![(1, 0), (2, 0), (2, 1), (2, 2), (2, 3), (1, 3)];
        assert_eq!(head_positions(input).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_move() {
        for i in -2..=2 {
            for j in -2..=2 {
                let actual = new_tail_pos(&(0, 0), (i, j));
                let expected = get_movements(&(i, j), &(0, 0));
                assert_eq!(expected, actual);
            }
        }
    }

    fn get_movements(
        leader_after_move: &(i32, i32),
        follower_before_move: &(i32, i32),
    ) -> (i32, i32) {
        let abs_delta = (
            ((leader_after_move.0 - follower_before_move.0).abs()),
            ((leader_after_move.1 - follower_before_move.1).abs()),
        );
        let mut movements: (i32, i32) = (
            leader_after_move.0 - follower_before_move.0,
            leader_after_move.1 - follower_before_move.1,
        );
        if abs_delta.0 != 2 && abs_delta.1 != 2 {
            movements = (0, 0)
        }
        if abs_delta.0 == 2 {
            movements.0 /= 2;
        }
        if abs_delta.1 == 2 {
            movements.1 /= 2;
        }

        movements
    }
}

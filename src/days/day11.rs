#![allow(unused)]

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};
use num::integer::lcm;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

pub fn day11() -> Result<(), color_eyre::eyre::Error> {
    day11a()?;
    day11b()?;
    Ok(())
}

fn day11a() -> Result<(), color_eyre::eyre::Error> {
    let input = include_str!("../../input/11.txt");
    let (_, mut monkeys) = parse_monkeys(input)?;

    for _ in 0..20 {
        simulate_round_a(&mut monkeys);
    }

    let business: usize = business(&monkeys);

    println!("Day11a: {business}");

    Ok(())
}

fn day11b() -> Result<(), color_eyre::eyre::Error> {
    let input = include_str!("../../input/11.txt");
    let (_, mut monkeys) = parse_monkeys(input)?;

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test)
        .reduce(lcm)
        .unwrap();

    for _ in 0..10000 {
        simulate_round_b(&mut monkeys, lcm);
    }

    let business: usize = business(&monkeys);

    println!("Day11b: {business}");

    Ok(())
}

fn business(monkeys: &[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|monkey| monkey.activity)
        .sorted()
        .rev()
        .take(2)
        .product()
}

struct Monkey {
    items: VecDeque<usize>,
    activity: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    test: usize,
    actions: (usize, usize),
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Monkey {{ items: {:?}, activity: {}, test: {}, actions: {:?} }}",
            self.items, self.activity, self.test, self.actions
        ))
    }
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = terminated(take_while(|c: char| c != '\n'), newline)(input)?;
    let (input, items) = terminated(parse_items, newline)(input)?;
    let (input, operation) = terminated(parse_operation, newline)(input)?;
    let (input, test) = terminated(parse_test, newline)(input)?;
    let (input, actions) = terminated(parse_actions, newline)(input)?;

    let monkey = Monkey {
        items: items.into(),
        activity: 0,
        operation: Box::new(operation),
        test,
        actions,
    };

    // println!("Parsed Monkey {monkey:?}");
    Ok((input, monkey))
}

fn parse_items(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, list) = separated_list0(tag(", "), nom::character::complete::u32)(input)?;
    let list = list.iter().map(|x| *x as usize).collect();
    Ok((input, list))
}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Old,
    Num(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum Symbol {
    Plus,
    Minus,
    Mul,
    Div,
}

fn parse_operation(input: &str) -> IResult<&str, impl Fn(usize) -> usize> {
    let (input, _) = tag("  Operation: new = ")(input)?;
    let (input, first) = parse_operand(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, operation) = parse_symbol(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, second) = parse_operand(input)?;

    let op = move |x| {
        let operation = match operation {
            Symbol::Plus => |a, b| a + b,
            Symbol::Minus => |a, b| a - b,
            Symbol::Mul => |a, b| a * b,
            Symbol::Div => |a, b| a / b,
        };
        let fst = match first {
            Operand::Old => x,
            Operand::Num(y) => y,
        };
        let snd = match second {
            Operand::Old => x,
            Operand::Num(y) => y,
        };

        operation(fst, snd)
    };
    Ok((input, op))
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map(nom::character::complete::u32, |x| Operand::Num(x as usize)),
    ))(input)
}

fn parse_symbol(input: &str) -> IResult<&str, Symbol> {
    alt((
        map(tag("+"), |_| Symbol::Plus),
        map(tag("-"), |_| Symbol::Minus),
        map(tag("*"), |_| Symbol::Mul),
        map(tag("/"), |_| Symbol::Div),
    ))(input)
}

fn parse_test(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    parse_usize(input)
}

fn parse_actions(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, first) = parse_usize(input)?;
    let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
    let (input, second) = parse_usize(input)?;

    Ok((input, (first, second)))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u32, |x| x as usize)(input)
}

fn simulate_round_a(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        // Collect items and where they go
        let mut items_to = vec![];
        {
            let monkey = &mut monkeys[i];
            while let Some(item) = monkey.items.pop_front() {
                let mut worry = (monkey.operation)(item);
                monkey.activity += 1;
                worry /= 3;
                let index = if worry % monkey.test == 0 {
                    monkey.actions.0
                } else {
                    monkey.actions.1
                };

                items_to.push((worry, index))
            }
        }
        // Move items where they go after the monkey is done with its items
        for (worry, index) in items_to {
            monkeys[index].items.push_back(worry);
        }
    }
}

fn simulate_round_b(monkeys: &mut [Monkey], lcm: usize) {
    for i in 0..monkeys.len() {
        // Collect items and where they go
        let mut items_to = vec![];
        {
            let monkey = &mut monkeys[i];
            while let Some(item) = monkey.items.pop_front() {
                let mut worry = (monkey.operation)(item);
                monkey.activity += 1;
                worry %= lcm;
                let index = if worry % monkey.test == 0 {
                    monkey.actions.0
                } else {
                    monkey.actions.1
                };

                items_to.push((worry, index))
            }
        }
        // Move items where they go after the monkey is done with its items
        for (worry, index) in items_to {
            monkeys[index].items.push_back(worry);
        }
    }
}

#[cfg(test)]
mod test {
    use nom::error::ErrorKind;
    use nom::Err;

    use super::*;

    #[test]
    fn test_parse_items() {
        let input = "  Starting items: 79, 98";
        let expected = Ok(("", vec![79, 98]));
        let result = parse_items(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_symbol() {
        let input = "+";
        let expected_output = Ok(("", Symbol::Plus));
        let result = parse_symbol(input);
        assert_eq!(result, expected_output);

        let input = "-";
        let expected_output = Ok(("", Symbol::Minus));
        let result = parse_symbol(input);
        assert_eq!(result, expected_output);

        let input = "*";
        let expected_output = Ok(("", Symbol::Mul));
        let result = parse_symbol(input);
        assert_eq!(result, expected_output);

        let input = "/";
        let expected_output = Ok(("", Symbol::Div));
        let result = parse_symbol(input);
        assert_eq!(result, expected_output);

        let input = "&";
        let result = parse_symbol(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_operand() {
        assert_eq!(parse_operand("old"), Ok(("", Operand::Old)));
        assert_eq!(parse_operand("5"), Ok(("", Operand::Num(5))));
        assert!(parse_operand("abc").is_err());
    }

    #[test]
    fn test_parse_test() {
        assert_eq!(parse_test("  Test: divisible by 23"), Ok(("", 23)));
    }

    #[test]
    fn test_parse_actions() {
        let input = "    If true: throw to monkey 2\n    If false: throw to monkey 3";
        let expected = Ok(("", (2, 3)));
        let actual = parse_actions(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_operation() {
        let input = "  Operation: new = old * 19";
        let actual = parse_operation(input);
        assert!(actual.is_ok(), "{}", actual.err().unwrap());
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n";
        let actual = parse_monkey(input);
        assert!(actual.is_ok(), "{}", actual.err().unwrap());
    }
}

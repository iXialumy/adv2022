#![allow(unused)]

use itertools::Itertools;
use miette::GraphicalReportHandler;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::newline;
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Offset, Parser};
use nom_locate::LocatedSpan;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree};
use nom_supreme::final_parser::final_parser;
use nom_supreme::parser_ext::ParserExt;
use num::integer::lcm;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

pub fn day11() {
    day11a();
    day11b();
}

fn day11a() {
    let input_static = include_str!("../../input/11.txt");
    let input = Span::new(input_static);
    let monkeys_res: Result<_, ErrorTree<Span>> =
        final_parser(parse_monkeys::<ErrorTree<Span>>)(input);

    let mut monkeys = match monkeys_res {
        Ok(monkeys) => monkeys,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input_static,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { base, contexts } => todo!(),
                GenericErrorTree::Alt(_) => todo!(),
            }
            return;
        }
    };

    for _ in 0..20 {
        simulate_round_a(&mut monkeys);
    }

    let business: usize = business(&monkeys);

    println!("Day11a: {business}");
}

fn day11b() {
    let input_static = include_str!("../../input/11.txt");
    let input = Span::new(input_static);
    let monkeys_res: Result<_, ErrorTree<Span>> =
        final_parser(parse_monkeys::<ErrorTree<Span>>)(input);

    let mut monkeys = match monkeys_res {
        Ok(monkeys) => monkeys,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input_static,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { base, contexts } => todo!(),
                GenericErrorTree::Alt(_) => todo!(),
            }
            return;
        }
    };

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

fn parse_monkeys<'a, E: ParseError<Span<'a>> + 'static>(
    input: Span<'a>,
) -> IResult<Span, Vec<Monkey>, E> {
    separated_list1(newline, parse_monkey)(input)
}

fn parse_monkey<'a, E: ParseError<Span<'a>> + 'static>(
    input: Span<'a>,
) -> IResult<Span<'a>, Monkey, E> {
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

fn parse_items<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Vec<usize>, E> {
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

fn parse_operation<'a, E: ParseError<Span<'a>>>(
    input: Span<'a>,
) -> IResult<Span<'a>, impl Fn(usize) -> usize, E> {
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

fn parse_operand<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Operand, E> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map(nom::character::complete::u32, |x| Operand::Num(x as usize)),
    ))(input)
}

fn parse_symbol<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Symbol, E> {
    alt((
        map(tag("+"), |_| Symbol::Plus),
        map(tag("-"), |_| Symbol::Minus),
        map(tag("*"), |_| Symbol::Mul),
        map(tag("/"), |_| Symbol::Div),
    ))(input)
}

fn parse_test<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, usize, E> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    parse_usize(input)
}

fn parse_actions<'a, E: ParseError<Span<'a>>>(
    input: Span<'a>,
) -> IResult<Span<'a>, (usize, usize), E> {
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, first) = parse_usize(input)?;
    let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
    let (input, second) = parse_usize(input)?;

    Ok((input, (first, second)))
}

fn parse_usize<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, usize, E> {
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

    type ErrType<'a> = ErrorTree<Span<'a>>;

    #[test]
    fn test_parse_items() {
        let input_static = "  Starting items: 79, 98";
        let input = Span::new(input_static);
        let expected = vec![79, 98];
        match parse_items::<ErrType>(input) {
            Ok((_, actual)) => assert_eq!(expected, actual),
            Result::Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn test_parse_symbol() {
        for (input, expected) in [
            ("+", Symbol::Plus),
            ("-", Symbol::Minus),
            ("*", Symbol::Mul),
            ("/", Symbol::Div),
        ] {
            let input = Span::new(input);
            match parse_symbol::<ErrType>(input) {
                Ok((_, actual)) => assert_eq!(expected, actual),
                Result::Err(e) => panic!("{e}"),
            }
        }

        let input = Span::new("&");
        let result = parse_symbol::<ErrType>(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_operand() {
        assert_eq!(
            parse_operand::<ErrType>(Span::new("old")).unwrap().1,
            Operand::Old
        );
        assert_eq!(
            parse_operand::<ErrType>(Span::new("5")).unwrap().1,
            Operand::Num(5)
        );
        assert!(parse_operand::<ErrType>(Span::new("abc")).is_err());
    }

    #[test]
    fn test_parse_test() {
        assert_eq!(
            parse_test::<ErrType>(Span::new("  Test: divisible by 23"))
                .unwrap()
                .1,
            23
        );
    }

    #[test]
    fn test_parse_actions() {
        let input = "    If true: throw to monkey 2\n    If false: throw to monkey 3";
        let expected = (2, 3);
        let actual = parse_actions::<ErrType>(Span::new(input)).unwrap().1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_operation() {
        let input = "  Operation: new = old * 19";
        let actual = parse_operation::<ErrType>(Span::new(input));
        assert!(actual.is_ok(), "{}", actual.err().unwrap());
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n";
        let actual = parse_monkey::<ErrType>(Span::new(input));
        assert!(actual.is_ok(), "{}", actual.err().unwrap());
    }
}

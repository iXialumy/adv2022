use std::{cmp::Ordering, convert::identity};
use std::ops::Deref;

use itertools::Itertools;
use miette::{GraphicalReportHandler, IntoDiagnostic};
use nom::{
    branch::alt,
    character::complete::{digit1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use nom_supreme::{error::StackContext, ParserExt};
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    final_parser::final_parser,
    tag::complete::tag,
};

pub fn day13() {
    // let input_raw = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
    let input_raw: &str = include_str!("../../input/13.txt");
    let a = day13a(input_raw);
    let b = day13b(input_raw);

    println!("Adv13a: {a}");
    println!("Adv13b: {b}");
}

fn day13a(input_raw: &str) -> usize {
    let input = Span::new(input_raw);
    let res: Result<_, ErrorTree<Span>> = final_parser(parse_pairs)(input);
    let pairs = match res {
        Ok(pairs) => pairs,
        Err(err) => {
            handle_error(input_raw, err);
            panic!();
        }
    };

    pairs
        .iter()
        .enumerate()
        .map(|(i, pair)| (i, pair.in_right_order()))
        .filter(|(i, cond)| *cond)
        .map(|(i, _)| i + 1) // increase by one because wanted indices begin at 1 not 0
        .sum()
}

fn day13b(input_raw: &str) -> usize {
    let input = Span::new(input_raw);
    let res: Result<_, ErrorTree<Span>> = final_parser(parse_lists)(input);
    let mut list = match res {
        Ok(list) => list,
        Err(err) => {
            handle_error(input_raw, err);
            panic!();
        }
    };
    
    let first_key = List(vec![ListEntry::List(List(vec![ListEntry::Num(2)]))]); // [[2]]
    let second_key = List(vec![ListEntry::List(List(vec![ListEntry::Num(6)]))]); // [[6]]
    
    list.push(first_key.clone());
    list.push(second_key.clone());
    list.sort();

    let first_key_index = list.binary_search(&first_key).unwrap() + 1;
    let second_key_index = list.binary_search(&second_key).unwrap() + 1;
    
    first_key_index * second_key_index
}

type Span<'a> = LocatedSpan<&'a str>;
type ErrType<'a> = ErrorTree<Span<'a>>;
type PResult<'a, T> = IResult<Span<'a>, T, ErrType<'a>>;

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("Could not Parse List structure")]
struct BadInput<'a> {
    #[source_code]
    src: &'a str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'a str, Box<dyn std::error::Error + Send + Sync>>,
}

#[derive(Debug)]
struct Pair(List, List);

impl From<(List, List)> for Pair {
    fn from(value: (List, List)) -> Self {
        Self(value.0, value.1)
    }
}

impl Pair {
    fn in_right_order(&self) -> bool {
        for (left, right) in self.0.iter().zip(self.1.iter()) {
            match left.cmp(right) {
                Ordering::Less => return true,
                Ordering::Equal => continue,
                Ordering::Greater => return false,
            }
        }
        // Must be equal now
        self.0.len().cmp(&self.1.len()) != Ordering::Greater
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct List(Vec<ListEntry>);

impl Deref for List {
    type Target = Vec<ListEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        for (left, right) in self.iter().zip(other.iter()) {
            match left.cmp(right) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Greater,
            }
        }
        // Must be equal now
        self.len().cmp(&other.len())
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListEntry {
    Num(u32),
    List(List),
}

impl Ord for ListEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListEntry::Num(a), ListEntry::Num(b)) => a.cmp(b),
            (ListEntry::List(a), ListEntry::List(b)) => {
                for (left, right) in a.iter().zip(b.iter()) {
                    match left.cmp(right) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                match a.len().cmp(&b.len()) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => Ordering::Equal,
                    Ordering::Greater => Ordering::Greater,
                }
            }
            (ListEntry::Num(_), ListEntry::List(_)) => {
                ListEntry::List(List(vec![self.to_owned()])).cmp(other)
            }
            (ListEntry::List(_), ListEntry::Num(_)) => {
                self.cmp(&ListEntry::List(List(vec![other.to_owned()])))
            }
        }
    }
}

impl PartialOrd for ListEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_pairs(input: Span) -> PResult<Vec<Pair>> {
    separated_list1(preceded(newline, newline), parse_pair)(input)
}

fn parse_pair(input: Span) -> PResult<Pair> {
    // println!("parsing pair");
    separated_pair(parse_list, newline, parse_list)
        .map(|pair| pair.into())
        .parse(input)
}

fn parse_lists(input: Span) -> PResult<Vec<List>> {
    separated_list1(newline, parse_optional_list)
        .map(|lists: Vec<Option<List>>| lists.into_iter().flatten().collect())
        .parse(input)
}

fn parse_optional_list(input: Span) -> PResult<Option<List>> {
    let filled_line = parse_list.map(Some);
    let empty_line = tag("").map(|_| None);
    alt((filled_line, empty_line))(input)
}

fn parse_list(input: Span) -> PResult<List> {
    // println!("parsing list");
    let (input, _) = tag("[")(input)?;
    let (input, list) = separated_list0(tag(","), parse_list_entry)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, List(list)))
}

fn parse_list_entry(input: Span) -> PResult<ListEntry> {
    // println!("parsing list entry");
    alt((parse_list_entry_list, parse_num))(input)
}

fn parse_num(input: Span) -> PResult<ListEntry> {
    // println!("parsing num");
    let res = nom::character::complete::u32(input);
    let (input, num) = res?;
    Ok((input, ListEntry::Num(num)))
}

fn parse_list_entry_list(input: Span) -> PResult<ListEntry> {
    // println!("parsing list entry list");
    let (input, list) = parse_list(input)?;
    Ok((input, ListEntry::List(list)))
}

fn handle_error(input: &str, err: ErrType) {
    match err {
        GenericErrorTree::Base { location, kind } => {
            let offset = location.location_offset();
            println!("{offset:?}");
            let err = BadInput {
                src: input,
                bad_bit: (offset, 1).into(),
                kind,
            };
            let mut s = String::new();
            GraphicalReportHandler::new()
                .render_report(&mut s, &err)
                .unwrap();
            println!("{s}");
        }
        GenericErrorTree::Stack { base, contexts } => {
            for (span, context) in contexts {
                match context {
                    StackContext::Kind(kind) => todo!(),
                    StackContext::Context(c) => todo!(),
                }
            }
        }
        GenericErrorTree::Alt(_) => todo!("Alt"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lists() {
        let input = Span::new("[]");
        let res = parse_list(input);
        assert!(res.is_ok());

        let input = Span::new("[[]]");
        let res = parse_list(input);
        assert!(res.is_ok());

        let input = Span::new("[[[]]]");
        let res = parse_list(input);
        assert!(res.is_ok());

        let input = Span::new("[1,1]");
        let res = parse_list(input);
        assert!(res.is_ok());

        let input = Span::new("[1,[]]");
        let res = parse_list(input);
        assert!(res.is_ok());

        let input = Span::new("[[[[8,7,8,5,4],6,[4,6]]],[6,[[]]],[]]");
        let res = parse_list(input);
        assert!(res.is_ok());
    }

    #[test]
    fn test_parse_list_entry_list() {
        let input = Span::new("[]");
        let res = parse_list_entry_list(input);
        assert!(res.is_ok());
    }

    #[test]
    fn test_parse_pair() {
        let input = Span::new("[[[[8,7,8,5,4],6,[4,6]]],[6,[[]]],[]]\n[[[[8,0,0,7,1],[1],8]]]");
        let res = parse_pair(input);
        assert!(res.is_ok());
    }

    #[test]
    fn test_parse_pairs() {
        let input = Span::new("[[[[8,7,8,5,4],6,[4,6]]],[6,[[]]],[]]\n[[[[8,0,0,7,1],[1],8]]]\n\n[[[5,8,[4,9,2],7,[8,0]],[9,5,[5,5]],[9,[3,3,8,4,8],[0,6]],5,3],[8,10,[[8,7,6,1,8],5,0,6,[5,1]],[7,[1],6],[6,2,1,[],[1,0,6,5]]],[1,1,[[2]]],[]]\n[[10,4,[],7,[[10,1]]],[[4,2]],[[[5],[],[],[]],[4,5,[2,4,9]]],[[8,[],[1,3,9,7]],4,7]]");
        let res = parse_pairs(input);
        assert!(res.is_ok());
        let (_, pairs) = res.unwrap();
        assert_eq!(2, pairs.len(), "{pairs:?}");
    }

    #[test]
    fn test_parse_input() {
        let input = Span::new(include_str!("../../input/13.txt"));
        let res = parse_pairs(input);
        assert!(res.is_ok());
        let (_, pairs) = res.unwrap();
        assert_eq!(150, pairs.len(), "{pairs:?}");
    }

    #[test]
    fn test_in_right_order() {
        let input = Span::new("[1,1,3,1,1]\n[1,1,5,1,1]");
        let (_, pair) = parse_pair(input).unwrap();
        assert!(pair.in_right_order());

        let input = Span::new("[[1],[2,3,4]]\n[[1],4]");
        let (_, pair) = parse_pair(input).unwrap();
        assert!(pair.in_right_order());

        let input = Span::new("[9]\n[[8,7,6]]");
        let (_, pair) = parse_pair(input).unwrap();
        assert!(!pair.in_right_order());

        let input = Span::new("[[4,4],4,4]\n[[4,4],4,4,4]");
        let (_, pair) = parse_pair(input).unwrap();
        assert!(pair.in_right_order());

        let input = Span::new("[[[]]]\n[[]]");
        let (_, pair) = parse_pair(input).unwrap();
        assert!(!pair.in_right_order());

        let input = Span::new("[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]");
        let (_, pair) = parse_pair(input).unwrap();
        assert!(!pair.in_right_order());
    }

    #[test]
    fn test_13a() {
        let input = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let expected = 13;
        let actual = day13a(input);
        assert_eq!(expected, actual);
    }
}

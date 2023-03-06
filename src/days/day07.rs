use std::iter::once;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_till1};
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{map_res, opt, recognize};
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

pub fn day07() {
    let input = include_str!("../../input/07.txt");
    let (_, root) = parse_non_exiting_cd(input).unwrap();
    day07a(&root);
    day07b(&root);
}

fn day07a(root: &Entry) {
    let sum: usize = root.sizes().filter(|size| *size <= 100000usize).sum();

    println!("Day07a: {sum}")
}

fn day07b(root: &Entry) {
    let total_space = 70000000;
    let used_space = root.size();
    let needed_space = 30000000 - (total_space - used_space);

    let min: usize = root
        .sizes()
        .filter(|size| *size >= needed_space)
        .min()
        .unwrap();

    println!("Day07b: {min}")
}

fn my_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn is_line_ending(c: char) -> bool {
    c == '\n' || c == '\r'
}

fn parse_file(input: &str) -> IResult<&str, Entry> {
    let (input, (size, name)) = terminated(
        separated_pair(my_usize, char(' '), take_till(is_line_ending)),
        line_ending,
    )(input)?;
    let entry = Entry::File { size, name };

    Ok((input, entry))
}

fn parse_folder(input: &str) -> IResult<&str, Entry> {
    let parse_line = preceded(tag("dir "), take_till(is_line_ending));
    let mut parse_with_line_ending = terminated(parse_line, line_ending);
    let (input, name) = parse_with_line_ending(input)?;
    let entry = Entry::Folder {
        name,
        entries: vec![],
    };
    Ok((input, entry))
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((parse_file, parse_folder))(input)
}

fn parse_ls(input: &str) -> IResult<&str, Vec<Entry>> {
    let (input, _) = terminated(tag("$ ls"), line_ending)(input)?;
    let (input, mut file_list) = many0(parse_entry)(input)?;
    file_list.retain(|entry| match entry {
        Entry::File { .. } => true,
        Entry::Folder { .. } => false,
    });
    let result = many0(parse_exiting_cd)(input); // Many0 does something i dont understand, which leads to the input being incorrectly consumed here
    let (input, mut folders) = result?;
    let (input, folder) = opt(parse_non_exiting_cd)(input)?;
    file_list.append(&mut folders);
    if let Some(entry) = folder {
        file_list.push(entry);
    }
    Ok((input, file_list))
}

fn parse_non_exiting_cd(input: &str) -> IResult<&str, Entry> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, folder_name) = take_till1(is_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, entries) = parse_ls(input)?;
    let entry = Entry::Folder {
        name: folder_name,
        entries,
    };
    Ok((input, entry))
}

fn parse_exiting_cd(input: &str) -> IResult<&str, Entry> {
    let (input, entry) = parse_non_exiting_cd(input)?;
    let (input, _) = terminated(tag("$ cd .."), line_ending)(input)?;
    Ok((input, entry))
}

#[derive(Debug, Eq, PartialEq)]
enum Entry<'a> {
    File {
        name: &'a str,
        size: usize,
    },
    Folder {
        name: &'a str,
        entries: Vec<Entry<'a>>,
    },
}

impl<'a> Entry<'a> {
    fn sizes(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.entries()
            .filter(|entry| match entry {
                Entry::File { .. } => false,
                Entry::Folder { .. } => true,
            })
            .map(|entry| entry.size())
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Entry::File { size, .. } => *size,
            Entry::Folder { entries, .. } => entries.iter().map(|entry| entry.size()).sum(),
        }
    }

    pub fn entries(&'a self) -> Box<dyn Iterator<Item = &Entry> + 'a> {
        match self {
            Entry::File { .. } => Box::new(once(self)),
            Entry::Folder { name: _, entries } => {
                Box::new(once(self).chain(entries.iter().flat_map(|entry| entry.entries())))
            }
        }
    }

    pub fn _pretty_print(&self) {
        self._pretty_print_("", true);
    }

    fn _pretty_print_(&self, indent: &str, last: bool) {
        let size = self.size();
        match self {
            Entry::File { name, .. } => {
                if last {
                    println!("{indent}└── {name}\t{size}");
                } else {
                    println!("{indent}├── {name}\t{size}")
                }
            }
            Entry::Folder { name, entries } => {
                if last {
                    println!("{indent}└── {name}\t{size}");
                } else {
                    println!("{indent}├── {name}\t{size}")
                }

                let add_indent = if last { "    " } else { "│   " };
                let indent = format!("{indent}{add_indent}");

                for (i, entry) in entries.iter().enumerate() {
                    entry._pretty_print_(&indent, i == entries.len() - 1)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::days::day07::*;

    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_file("14848514 b.txt\n"),
            Ok((
                "",
                Entry::File {
                    name: "b.txt",
                    size: 14848514
                }
            ))
        );
    }

    #[test]
    fn test_parse_folder() {
        assert_eq!(
            parse_folder("dir a\n"),
            Ok((
                "",
                Entry::Folder {
                    name: "a",
                    entries: vec![]
                }
            ))
        );
    }

    #[test]
    fn test_parse_entry() {
        assert_eq!(
            parse_entry("678912345 asdf\n"),
            Ok((
                "",
                Entry::File {
                    name: "asdf",
                    size: 678912345
                }
            ))
        );

        assert_eq!(
            parse_folder("dir f\n"),
            Ok((
                "",
                Entry::Folder {
                    name: "f",
                    entries: vec![]
                }
            ))
        );
    }

    #[test]
    fn test_parse_ls() {
        assert_eq!(
            parse_ls("$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n"),
            Ok((
                "",
                vec![
                    Entry::File {
                        name: "j",
                        size: 4060174
                    },
                    Entry::File {
                        name: "d.log",
                        size: 8033020
                    },
                    Entry::File {
                        name: "d.ext",
                        size: 5626152
                    },
                    Entry::File {
                        name: "k",
                        size: 7214296
                    },
                ]
            ))
        );
    }

    #[test]
    fn test_parse_non_exiting_cd() {
        let input = "$ cd d\n$ ls\n4060174 b\n8033020 d.log\n5626152 d.ext\n7214296 k\n";
        assert_eq!(
            parse_non_exiting_cd(input),
            Ok((
                "",
                Entry::Folder {
                    name: "d",
                    entries: vec![
                        Entry::File {
                            name: "b",
                            size: 4060174
                        },
                        Entry::File {
                            name: "d.log",
                            size: 8033020
                        },
                        Entry::File {
                            name: "d.ext",
                            size: 5626152
                        },
                        Entry::File {
                            name: "k",
                            size: 7214296
                        },
                    ],
                }
            ))
        );

        let input = "$ cd /\n$ ls\ndir a\n$ cd a\n$ ls\n123 b.txt\n";
        assert_eq!(
            parse_non_exiting_cd(input),
            Ok((
                "",
                Entry::Folder {
                    name: "/",
                    entries: vec![Entry::Folder {
                        name: "a",
                        entries: vec![Entry::File {
                            name: "b.txt",
                            size: 123
                        }],
                    }],
                }
            ))
        );
    }

    #[test]
    fn test_parse_exiting_cd() {
        let input = "$ cd d\n$ ls\n4060174 a\n8033020 d.log\n5626152 d.ext\n7214296 k\n$ cd ..\n";
        assert_eq!(
            parse_exiting_cd(input),
            Ok((
                "",
                Entry::Folder {
                    name: "d",
                    entries: vec![
                        Entry::File {
                            name: "a",
                            size: 4060174
                        },
                        Entry::File {
                            name: "d.log",
                            size: 8033020
                        },
                        Entry::File {
                            name: "d.ext",
                            size: 5626152
                        },
                        Entry::File {
                            name: "k",
                            size: 7214296
                        },
                    ],
                }
            ))
        );
    }
}

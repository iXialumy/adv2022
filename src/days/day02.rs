pub fn day02() {
    let input = include_str!("../../input/02.txt");
    day02a(input);
    day02b(input);
}

#[derive(Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn play(&self, other: &Self) -> GameEnd {
        match (self, other) {
            (Self::Rock, Self::Rock) => GameEnd::Draw,
            (Self::Rock, Self::Paper) => GameEnd::Lose,
            (Self::Rock, Self::Scissors) => GameEnd::Win,
            (Self::Paper, Self::Rock) => GameEnd::Win,
            (Self::Paper, Self::Paper) => GameEnd::Draw,
            (Self::Paper, Self::Scissors) => GameEnd::Lose,
            (Self::Scissors, Self::Rock) => GameEnd::Lose,
            (Self::Scissors, Self::Paper) => GameEnd::Win,
            (Self::Scissors, Self::Scissors) => GameEnd::Draw,
        }
    }

    fn from_char(c: &char) -> Option<Self> {
        match c {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissors),
            _ => None,
        }
    }
}

enum GameEnd {
    Win,
    Draw,
    Lose,
}

impl GameEnd {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            'X' => Some(Self::Lose),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}

fn day02a(input: &str) {
    let sum: u32 = input
        .lines()
        .flat_map(parse_line_a)
        .map(|x| points_for_round(&x))
        .sum();

    println!("Day02a: {sum}");
}

fn day02b(input: &str) {
    let sum: u32 = input
        .lines()
        .flat_map(parse_line_b)
        .map(correct_play)
        .map(|x| points_for_round(&x))
        .sum();

    println!("Day02b: {sum}");
}

fn parse_line_a(line: &str) -> Option<(HandShape, HandShape)> {
    Some((
        HandShape::from_char(&line.chars().next()?)?,
        HandShape::from_char(&line.chars().nth(2)?)?,
    ))
}

fn parse_line_b(line: &str) -> Option<(HandShape, GameEnd)> {
    Some((
        HandShape::from_char(&line.chars().next()?)?,
        GameEnd::from_char(&line.chars().nth(2)?)?,
    ))
}

fn points_for_round((other, me): &(HandShape, HandShape)) -> u32 {
    let shape_score = match me {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    };

    let game_score = match me.play(other) {
        GameEnd::Win => 6,
        GameEnd::Draw => 3,
        GameEnd::Lose => 0,
    };

    shape_score + game_score
}

fn correct_play((other, me): (HandShape, GameEnd)) -> (HandShape, HandShape) {
    let play = match (other, me) {
        (HandShape::Rock, GameEnd::Win) => HandShape::Paper,
        (HandShape::Rock, GameEnd::Draw) => HandShape::Rock,
        (HandShape::Rock, GameEnd::Lose) => HandShape::Scissors,
        (HandShape::Paper, GameEnd::Win) => HandShape::Scissors,
        (HandShape::Paper, GameEnd::Draw) => HandShape::Paper,
        (HandShape::Paper, GameEnd::Lose) => HandShape::Rock,
        (HandShape::Scissors, GameEnd::Win) => HandShape::Rock,
        (HandShape::Scissors, GameEnd::Draw) => HandShape::Scissors,
        (HandShape::Scissors, GameEnd::Lose) => HandShape::Paper,
    };

    (other, play)
}

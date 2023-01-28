use color_eyre::eyre::Result;

use days::day01::*;
use days::day02::*;
use days::day03::*;
use days::day04::*;
use days::day05::*;
use days::day06::*;
use days::day07::*;
use days::day08::*;
#[allow(unused)]
use days::day09::*;

mod days;

fn main() -> Result<()> {
    color_eyre::install()?;
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    // day09();
    Ok(())
}

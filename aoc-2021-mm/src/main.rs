#![feature(lint_reasons)]
#![feature(iter_array_chunks)]
#![expect(unused_variables)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod warmup_2022_day_02;

use day_01::Day01;
use day_02::Day02;
use day_03::Day03;
use day_04::Day04;
use day_05::Day05;
use day_06::Day06;
use day_07::Day07;
use day_08::Day08;
use day_09::Day09;
use day_10::Day10;
use day_11::Day11;
use day_12::Day12;
use day_13::Day13;
use day_14::Day14;
use day_15::Day15;
use day_16::Day16;
use day_17::Day17;
use day_18::Day18;
use day_19::Day19;
use day_20::Day20;
use day_21::Day21;
use day_22::Day22;
use day_23::Day23;
use day_24::Day24;
use day_25::Day25;
use warmup_2022_day_02::Warmup2022Day02;

use std::error::Error;

use aoc_framework::{check_solved_tasks, BoxedAocTask};
type BoxedError = Box<dyn Error + Send + Sync>;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let tasks: Vec<BoxedAocTask> = vec![
        Box::new(Warmup2022Day02),
        Box::new(Day01),
        Box::new(Day02),
        Box::new(Day03),
        Box::new(Day04),
        Box::new(Day05),
        Box::new(Day06),
        Box::new(Day07),
        Box::new(Day08),
        Box::new(Day09),
        Box::new(Day10),
        Box::new(Day11),
        Box::new(Day12),
        Box::new(Day13),
        Box::new(Day14),
        Box::new(Day15),
        Box::new(Day16),
        Box::new(Day17),
        Box::new(Day18),
        Box::new(Day19),
        Box::new(Day20),
        Box::new(Day21),
        Box::new(Day22),
        Box::new(Day23),
        Box::new(Day24),
        Box::new(Day25),
    ];

    check_solved_tasks(tasks, 2)?;

    Ok(())
}

use std::{ops::Add, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocInput, AocSolution, AocTask};
use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;

pub struct RockPaperScissors;

#[derive(Clone, PartialEq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Hand {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(eyre!("Invalid Hand: {value}")),
        }
    }
}

impl From<&Strategy> for Hand {
    fn from(value: &Strategy) -> Self {
        match value {
            Strategy::X => Self::Rock,
            Strategy::Y => Self::Paper,
            Strategy::Z => Self::Scissors,
        }
    }
}

impl Hand {
    fn vs(&self, other: &Hand) -> i32 {
        match (self, other) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => 6,
            (a, b) if a == b => 3,
            _ => 0,
        }
    }

    fn value(self) -> i32 {
        self as i32 + 1
    }
}

#[derive(PartialEq, Debug)]
enum Strategy {
    X,
    Y,
    Z,
}

impl TryFrom<char> for Strategy {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::X),
            'Y' => Ok(Self::Y),
            'Z' => Ok(Self::Z),
            _ => Err(eyre!("Invalid Strategy: {value}")),
        }
    }
}

#[derive(Debug)]
enum StrategyType {
    Hand,
    Result,
}

#[derive(PartialEq, Debug)]
struct Round {
    opponent: Hand,
    strategy: Strategy,
}

impl TryFrom<String> for Round {
    type Error = Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (opponent, strategy) = value
            .split_whitespace()
            .collect::<String>()
            .chars()
            .collect_tuple()
            .ok_or(eyre!("Invalid Round: {value}"))?;

        Ok(Round {
            opponent: opponent.try_into()?,
            strategy: strategy.try_into()?,
        })
    }
}

impl Round {
    fn score(&self, strategy: &StrategyType) -> i32 {
        match strategy {
            StrategyType::Hand => self.score_as_hand(),
            StrategyType::Result => self.score_as_result(),
        }
    }

    fn score_as_hand(&self) -> i32 {
        let my_hand = Hand::from(&self.strategy);
        my_hand.vs(&self.opponent) + my_hand.value()
    }

    fn score_as_result(&self) -> i32 {
        let my_hand = match (&self.opponent, &self.strategy) {
            (a, Strategy::Y) => a.clone(),
            (Hand::Rock, Strategy::X) => Hand::Scissors,
            (Hand::Rock, Strategy::Z) => Hand::Paper,
            (Hand::Paper, Strategy::X) => Hand::Rock,
            (Hand::Paper, Strategy::Z) => Hand::Scissors,
            (Hand::Scissors, Strategy::X) => Hand::Paper,
            (Hand::Scissors, Strategy::Z) => Hand::Rock,
        };
        my_hand.vs(&self.opponent) + my_hand.value()
    }
}

impl AocTask for RockPaperScissors {
    fn directory(&self) -> PathBuf {
        "tasks/warmup_2022_rock_paper_scissors".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        let strategy = match phase {
            1 => StrategyType::Hand,
            2 => StrategyType::Result,
            p => Err(eyre!("Invalid phase: {p}"))?,
        };

        input
            .map(|r| r.map_err(|err| err.into()).and_then(Round::try_from))
            .map_ok(|round| round.score(&strategy))
            .fold_ok(0, Add::add)
            .try_solved()
    }
}

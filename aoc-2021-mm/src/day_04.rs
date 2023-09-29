use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;

pub struct Day04;

#[derive(Debug, Default)]
struct Board {
    data: Vec<i32>,
    hits: Vec<bool>,
}

impl<'board> Board {
    fn bingo(&self) -> bool {
        let rows = (0..5).map(|row| self.hits_row(row).all(|hit| *hit));
        let cols = (0..5).map(|col| self.hits_col(col).all(|hit| *hit));
        let bingo = rows.chain(cols).find(|bingo| *bingo);
        matches!(bingo, Some(true))
    }

    fn play(&mut self, ball: &i32) {
        self.data
            .iter()
            .positions(|number| number == ball)
            .for_each(|hit| self.hits[hit] = true);
    }

    fn hits_row(&'board self, idx: usize) -> impl Iterator<Item = &bool> + 'board {
        self.hits.iter().skip(idx * 5).take(5)
    }

    fn hits_col(&'board self, idx: usize) -> impl Iterator<Item = &bool> + 'board {
        self.hits.iter().skip(idx).step_by(5)
    }
}

#[derive(Debug, Default)]
struct BingoRoundIterator {
    boards: Vec<Board>,
    balls: Vec<i32>,
}

impl Iterator for BingoRoundIterator {
    type Item = Option<usize>;

    // Plays one round of bingo, returns a winner if there is one
    fn next(&mut self) -> Option<Self::Item> {
        let ball = self.balls.pop()?;

        let winner = self
            .boards
            .iter_mut()
            .update(|board| board.play(&ball))
            .enumerate()
            .find(|(i, board)| board.bingo());

        match winner {
            Some(winner) => Some(Some(winner.0)),
            None => Some(None),
        }
    }
}

impl TryFrom<[String; 5]> for Board {
    type Error = Report;

    fn try_from(value: [String; 5]) -> Result<Self, Report> {
        let data = value
            .into_iter()
            .flat_map(|row| {
                row.split_whitespace()
                    .map(|num| num.parse())
                    .collect::<Vec<_>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if data.len() != 25 {
            Err(eyre!("Invalid board definition"))
        } else {
            Ok(Self {
                data,
                hits: Default::default(),
            })
        }
    }
}

impl AocTask for Day04 {
    fn directory(&self) -> PathBuf {
        "tasks/day_04".into()
    }

    fn solution(&self, mut input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let balls: Result<Vec<i32>, _> = input
            .next()
            .ok_or(eyre!("Missing input"))?
            .split(',')
            .map(|ball| ball.parse())
            .rev()
            .collect();

        let boards: Result<Vec<Board>, _> = input
            .filter(|line| line.is_empty())
            .array_chunks()
            .map(Board::try_from)
            .collect();

        let bingo = BingoRoundIterator {
            balls: balls?,
            boards: boards?,
        };

        println!("{bingo:#?}");

        0.solved()
    }
}

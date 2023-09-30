use std::{collections::HashMap, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;

pub struct Day04;

#[derive(Debug, Default, Clone)]
struct Board {
    data: Vec<i32>,
    hits: Vec<bool>,
}

impl<'board> Board {
    fn bingo(&self) -> bool {
        let rows = (0..5).map(|row| self.hits_row(row));
        let cols = (0..5).map(|col| self.hits_col(col));
        let bingo = rows
            .chain(cols)
            .find_map(|mut hit_iter| hit_iter.all(|&hit| hit).then_some(())); // Find the first bingo
        bingo.is_some()
    }

    fn play(&mut self, ball: &i32) {
        self.data
            .iter()
            .positions(|number| number == ball)
            .for_each(|hit| self.hits[hit] = true);
    }

    fn score(&self) -> i32 {
        self.data
            .iter()
            .zip(self.hits.iter())
            .fold(0, |acc, (number, hit)| match hit {
                false => acc + number,
                true => acc,
            })
    }

    fn hits_row(&'board self, idx: usize) -> Box<dyn Iterator<Item = &bool> + 'board> {
        Box::new(self.hits.iter().skip(idx * 5).take(5))
    }

    fn hits_col(&'board self, idx: usize) -> Box<dyn Iterator<Item = &bool> + 'board> {
        Box::new(self.hits.iter().skip(idx).step_by(5))
    }
}

#[derive(Debug, Default)]
struct BingoRoundIterator {
    boards: HashMap<usize, Board>,
    balls: Vec<i32>,
}

impl Iterator for BingoRoundIterator {
    type Item = (Vec<Board>, i32);

    // Plays one round of bingo, returns the winning boards and the ball that was played
    fn next(&mut self) -> Option<Self::Item> {
        let ball = self.balls.pop()?;

        let winner_ids: Vec<_> = self
            .boards
            .iter_mut()
            .update(|(_, board)| board.play(&ball))
            .filter_map(|(&index, board)| board.bingo().then_some(index))
            .collect();

        let winners: Vec<Board> = winner_ids
            .into_iter()
            .map(|winner| self.boards.remove(&winner).unwrap()) // Safe unwrap
            .collect();

        match winners.len() {
            0 => Some((vec![], ball)),
            _ => Some((winners, ball)),
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
                hits: vec![false; 25],
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

        let boards: HashMap<usize, Board> = input
            .filter(|line| !line.is_empty())
            .array_chunks()
            .map(Board::try_from)
            .process_results(|results| results.enumerate().collect())?;

        let bingo_iterator = BingoRoundIterator {
            balls: balls?,
            boards,
        };

        let mut bingos = bingo_iterator.filter(|(winners, ball)| !winners.is_empty());

        // Safe to unwarp here since the empty vectors have been filtered out
        let (bingo_score, last_ball) = match phase {
            1 => bingos
                .next()
                .map(|(winners, ball)| (winners.first().unwrap().score(), ball)),
            2 => bingos
                .last()
                .map(|(winners, ball)| (winners.last().unwrap().score(), ball)),
            _ => todo!(),
        }
        .ok_or(eyre!("No bingos found"))?;

        (bingo_score * last_ball).solved()
    }
}

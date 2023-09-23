use std::{
    ops::{Add, Mul},
    path::PathBuf,
};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::eyre::{bail, eyre, Context};
use itertools::Itertools;

pub struct Dive;

#[derive(Default)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    fn product(self) -> i32 {
        self.x * self.y
    }
}

impl Add<Move> for Position {
    type Output = Self;

    fn add(mut self, rhs: Move) -> Self::Output {
        match rhs {
            Move::Forward(dist) => self.x += dist,
            Move::Down(dist) => self.y += dist,
            Move::Up(dist) => self.y -= dist,
        }
        self
    }
}

impl Mul<Move> for Position {
    type Output = Position;

    fn mul(mut self, rhs: Move) -> Self::Output {
        match rhs {
            Move::Forward(dist) => {
                self.x += dist;
                self.y += self.aim * dist
            }
            Move::Down(dist) => self.aim += dist,
            Move::Up(dist) => self.aim -= dist,
        }
        self
    }
}

enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl TryFrom<String> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (direction, distance) = value
            .split_whitespace()
            .collect_tuple()
            .ok_or(eyre!("Invalid Move"))?;
        let distance: i32 = distance.parse().context("Invalid distance")?;

        let r#move = match direction {
            "forward" => Self::Forward(distance),
            "down" => Self::Down(distance),
            "up" => Self::Up(distance),
            _ => bail!("Invalid direction"),
        };
        Ok(r#move)
    }
}

impl AocTask for Dive {
    fn directory(&self) -> PathBuf {
        "tasks/02_dive".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let op = match phase {
            1 => Add::add,
            2 => Mul::mul,
            _ => todo!(),
        };

        input
            .map(Move::try_from)
            .fold_ok(Position::default(), op)?
            .product()
            .solved()
    }
}

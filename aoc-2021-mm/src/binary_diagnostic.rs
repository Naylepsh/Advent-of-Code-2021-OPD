use std::{collections::HashMap, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;

pub struct BinaryDiagnostic;

struct Diagnostic {
    data: Vec<char>,
    bits: usize,
    counts: Vec<HashMap<char, usize>>,
}

impl<'diag> Diagnostic {
    fn new(data: Vec<Vec<char>>) -> Result<Self, Report> {
        // # of bits in each number
        let first = data.first();
        let bits = first
            .map(|number| number.len())
            .ok_or(eyre!("Invalid number: {first:#?}"))?;

        let mut diagnostic = Self {
            data: data.into_iter().flatten().collect(),
            bits,
            counts: vec![],
        };
        diagnostic.count_ones();
        Ok(diagnostic)
    }

    fn column(&'diag self, idx: usize) -> impl Iterator<Item = char> + 'diag {
        self.data.iter().skip(idx).step_by(self.bits).copied()
    }

    fn count_ones(&mut self) {
        self.counts = (0..self.bits)
            .map(|col| self.column(col).counts())
            .collect();
    }

    // Min (epsilon), Max (gamma).
    fn column_epsilon_gamma(&'diag self, idx: usize) -> Result<(&'diag char, &'diag char), Report> {
        match self.counts[idx]
            .iter()
            .minmax_by(|left, right| left.1.cmp(right.1))
        {
            // From phase 2: draw => min = 0, max = 1
            itertools::MinMaxResult::OneElement(_) => Ok((&'0', &'1')),
            itertools::MinMaxResult::MinMax(min, max) => Ok((min.0, max.0)),
            itertools::MinMaxResult::NoElements => Err(eyre!("Invalid input")),
        }
    }

    fn power_consumption(&self) -> Result<i32, Report> {
        let (epsilon, gamma): (String, String) = (0..self.bits)
            .map(|idx| self.column_epsilon_gamma(idx))
            .process_results(|iter| iter.unzip())?;

        Ok(i32::from_str_radix(&epsilon, 2)? * i32::from_str_radix(&gamma, 2)?)
    }
}

impl AocTask for BinaryDiagnostic {
    fn directory(&self) -> PathBuf {
        "tasks/03_binary_diagnostic".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let diagnostic = Diagnostic::new(input.map(|str| str.chars().collect()).collect())?;

        match phase {
            1 => diagnostic.power_consumption()?,
            2 => 0,
            _ => todo!(),
        }
        .solved()
    }
}

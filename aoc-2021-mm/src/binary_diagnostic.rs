use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;

pub struct BinaryDiagnostic;

struct Diagnostic {
    data: Vec<char>,
    bits: usize,
}

impl<'data> Diagnostic {
    fn new(data: Vec<Vec<char>>) -> Result<Self, Report> {
        // # of bits in each number
        let first = data.first();
        let bits = first
            .map(|number| number.len())
            .ok_or(eyre!("Invalid number: {first:#?}"))?;

        Ok(Self {
            data: data.into_iter().flatten().collect(),
            bits,
        })
    }

    fn column(
        &self,
        iter: impl Iterator<Item = &'data char> + 'data,
        idx: usize,
    ) -> impl Iterator<Item = char> + 'data {
        iter.skip(idx).step_by(self.bits).copied()
    }

    fn row_vec(&self) -> Vec<Vec<char>> {
        self.data
            .iter()
            .copied()
            .chunks(self.bits)
            .into_iter()
            .map(|chunk| chunk.collect())
            .collect()
    }

    // Min (epsilon), Max (gamma).
    fn epsilon_gamma(
        &self,
        data: impl Iterator<Item = char>,
        override_equal: bool,
    ) -> Result<(char, char), Report> {
        match data
            .counts()
            .iter()
            .minmax_by(|left, right| left.1.cmp(right.1))
        {
            // From phase 2: draw => min = 0, max = 1
            itertools::MinMaxResult::NoElements => Err(eyre!("Invalid input")),
            itertools::MinMaxResult::OneElement(_) => Err(eyre!("Invalid input")),
            itertools::MinMaxResult::MinMax(min, max) if min == max && override_equal => {
                Ok(('1', '0'))
            }
            itertools::MinMaxResult::MinMax(min, max) => Ok((min.0.clone(), max.0.clone())),
        }
    }

    fn power_consumption(&self) -> Result<i32, Report> {
        let (epsilon, gamma): (String, String) = (0..self.bits)
            .map(|idx| self.epsilon_gamma(self.column(self.data.iter(), idx), false))
            .process_results(|iter| iter.unzip())?;

        Ok(i32::from_str_radix(&epsilon, 2)? * i32::from_str_radix(&gamma, 2)?)
    }

    fn life_support(&self) -> Result<i32, Report> {
        let mut oxygen: Vec<Vec<char>> = self.row_vec();
        let mut co2: Vec<Vec<char>> = self.row_vec();

        for idx in 0..self.bits {
            if oxygen.len() > 1 {
                let (_, ox_gamma) =
                    self.epsilon_gamma(self.column(oxygen.iter().flatten(), idx), true)?;
                oxygen = oxygen
                    .into_iter()
                    .filter(|row| row.get(idx) == Some(&ox_gamma))
                    .collect();
            }

            if co2.len() > 1 {
                let (co2_epsilon, _) =
                    self.epsilon_gamma(self.column(co2.iter().flatten(), idx), true)?;
                co2 = co2
                    .into_iter()
                    .filter(|row| row.get(idx) == Some(&co2_epsilon))
                    .collect();
            }

            if oxygen.len() == 1 && co2.len() == 1 {
                break;
            }
        }

        let oxygen_rating: String = oxygen
            .first()
            .ok_or(eyre!("Invalid input"))?
            .iter()
            .collect();
        let co2_scrubber_rating: String =
            co2.first().ok_or(eyre!("Invalid input"))?.iter().collect();

        Ok(i32::from_str_radix(&oxygen_rating, 2)? * i32::from_str_radix(&co2_scrubber_rating, 2)?)
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
            2 => diagnostic.life_support()?,
            _ => todo!(),
        }
        .solved()
    }
}

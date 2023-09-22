use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct DiracDice;

impl AocTask for DiracDice {
    fn directory(&self) -> PathBuf {
        "tasks/21_dirac_dice".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}

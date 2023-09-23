use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct DiracDice;

impl AocTask for DiracDice {
    fn directory(&self) -> PathBuf {
        "tasks/21_dirac_dice".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

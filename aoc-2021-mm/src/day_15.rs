use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct Day15;

impl AocTask for Day15 {
    fn directory(&self) -> PathBuf {
        "tasks/day_15".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

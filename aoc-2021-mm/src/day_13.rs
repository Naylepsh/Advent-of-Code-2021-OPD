use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct Day13;

impl AocTask for Day13 {
    fn directory(&self) -> PathBuf {
        "tasks/day_13".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

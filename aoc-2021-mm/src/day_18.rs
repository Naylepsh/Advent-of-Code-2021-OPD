use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct Day18;

impl AocTask for Day18 {
    fn directory(&self) -> PathBuf {
        "tasks/day_18".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

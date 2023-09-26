use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct Day14;

impl AocTask for Day14 {
    fn directory(&self) -> PathBuf {
        "tasks/day_14".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

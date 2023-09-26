use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct Day23;

impl AocTask for Day23 {
    fn directory(&self) -> PathBuf {
        "tasks/day_23".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

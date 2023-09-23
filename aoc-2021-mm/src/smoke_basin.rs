use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct SmokeBasin;

impl AocTask for SmokeBasin {
    fn directory(&self) -> PathBuf {
        "tasks/09_smoke_basin".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

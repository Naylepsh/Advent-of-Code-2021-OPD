use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct GiantSquid;

impl AocTask for GiantSquid {
    fn directory(&self) -> PathBuf {
        "tasks/04_giant_squid".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

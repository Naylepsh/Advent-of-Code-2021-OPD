use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct PassagePathing;

impl AocTask for PassagePathing {
    fn directory(&self) -> PathBuf {
        "tasks/12_passage_pathing".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

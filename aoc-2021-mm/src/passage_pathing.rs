use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct PassagePathing;

impl AocTask for PassagePathing {
    fn directory(&self) -> PathBuf {
        "tasks/12_passage_pathing".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}

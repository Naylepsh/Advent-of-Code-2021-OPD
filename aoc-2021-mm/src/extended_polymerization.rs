use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct ExtendedPolymerization;

impl AocTask for ExtendedPolymerization {
    fn directory(&self) -> PathBuf {
        "tasks/14_extended_polymerization".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}

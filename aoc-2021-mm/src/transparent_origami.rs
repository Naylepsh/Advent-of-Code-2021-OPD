use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct TransparentOrigami;

impl AocTask for TransparentOrigami {
    fn directory(&self) -> PathBuf {
        "tasks/13_transparent_origami".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
